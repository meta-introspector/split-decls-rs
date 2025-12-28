macro_rules! deps {
    () => {
        Config!();
        Args!();
    };
}

macro_rules! handle_plan_mode {
    () => {
        deps!();
        pub async fn handle_plan_mode (_args : & Args , _config : & Config) -> Result < () > { println ! ("Planning mode activated. This will list available tasks and their estimated sizes.") ; Ok (()) }
    };
}

handle_plan_mode!();