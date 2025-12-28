macro_rules! deps {
    () => {
        Args!();
        Config!();
    };
}

macro_rules! handle_run_pipeline {
    () => {
        deps!();
        pub async fn handle_run_pipeline (args : & crate :: Args , config : & Config) -> anyhow :: Result < () > { println ! ("Running main pipeline...") ; let mut stdout = tokio :: io :: stdout () ; let dummy_content = "fn main() { println!(\"Hello, world!\"); }" . to_string () ; let dummy_path = "dummy_file.rs" . to_string () ; pipeline :: run_category_pipeline (& mut stdout , & dummy_content , & dummy_path , & args , & Some (config . clone ()) ,) . await ? ; Ok (()) }
    };
}

handle_run_pipeline!()