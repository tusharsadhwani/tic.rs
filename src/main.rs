use std::io::Write;

fn main() {
    print_instructions();

    let (player_1, player_2) = choose_players();
    let mut board: [String; 9] = core::array::from_fn(|_| " ".to_string());

    loop {
        let game_over = play_turn(&mut board, &player_1);
        if game_over {
            break;
        }
        let game_over = play_turn(&mut board, &player_2);
        if game_over {
            break;
        }
    }
}

fn print_instructions() {
    println!(
        "Welcome to Tic Tac Toe.\n\
        Board positions are as follows:",
    );
    let positions: [String; 9] = core::array::from_fn(|i| (i + 1).to_string());
    print_board(&positions);
}

fn print_board(positions: &[String; 9]) {
    println!(
        r#"
         {} | {} | {} 
        -----------
         {} | {} | {} 
        -----------
         {} | {} | {} 
        "#,
        positions[0],
        positions[1],
        positions[2],
        positions[3],
        positions[4],
        positions[5],
        positions[6],
        positions[7],
        positions[8],
    );
}

fn choose_players() -> (String, String) {
    let player_1 = loop {
        print!("Player 1, choose your symbol (X or O): ");
        std::io::stdout().flush().unwrap();

        let mut player_1 = String::new();
        std::io::stdin()
            .read_line(&mut player_1)
            .expect("Failed to read line");

        let player_1 = player_1.trim().to_uppercase();
        if player_1 == "X" || player_1 == "O" {
            break player_1;
        } else {
            println!("Invalid symbol. Please choose X or O.");
        }
    };
    let player_2 = if player_1 == "X" { "O" } else { "X" }.to_string();
    return (player_1, player_2);
}

fn play_turn(board: &mut [String; 9], player: &String) -> bool {
    print_board(board);

    loop {
        print!(
            "Player {}, choose a position to place your symbol: ",
            player
        );
        std::io::stdout().flush().unwrap();

        let mut position = String::new();
        std::io::stdin()
            .read_line(&mut position)
            .expect("Failed to read line");

        let position: usize = match position.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid position. Please choose a number between 1 and 9.");
                continue;
            }
        };

        if position < 1 || position > 9 {
            println!("Invalid position. Please choose a number between 1 and 9.");
            continue;
        }

        let position = position - 1;
        if board[position] != " " {
            println!("Position already taken. Please choose an empty position.");
            continue;
        }

        board[position] = player.to_string();

        if check_winner(board, player) {
            print_board(board);
            println!("Player {} wins!", player);
            return true;
        }

        if check_tie(board) {
            print_board(board);
            println!("It's a draw!");
            return true;
        }

        break;
    }

    return false;
}

fn check_winner(board: &[String; 9], player: &String) -> bool {
    let winning_combinations: [[usize; 3]; 8] = [
        // rows
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        // columns
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        // diagonals
        [0, 4, 8],
        [2, 4, 6],
    ];

    for combination in winning_combinations.iter() {
        if combination
            .iter()
            .all(|&position| &board[position] == player)
        {
            return true;
        }
    }

    return false;
}

fn check_tie(board: &[String; 9]) -> bool {
    return board.iter().all(|char| char != " ");
}
