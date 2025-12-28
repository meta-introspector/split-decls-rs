macro_rules! deps {
    () => {
        Cli!();
    };
}

macro_rules! run_process_tt_txt_command {
    () => {
        deps!();
        pub fn run_process_tt_txt_command (args : & ProcessTtTxtArgs , cli : & Cli) -> Result < () > { println ! ("Processing tt.txt file: {:?}" , args . tt_txt_path) ; Ok (()) }
    };
}

run_process_tt_txt_command!();