macro_rules! CargoCommand {
    () => {
        pub trait CargoCommand { fn needs_execution (& self , current_dir : & Path , executor : Arc < dyn Execv + Send + Sync > ,) -> anyhow :: Result < bool > ; fn execute (& self , current_dir : & Path , log_file : & mut File , executor : Arc < dyn Execv + Send + Sync > ,) -> anyhow :: Result < Output > ; fn dry_run (& self , current_dir : & Path , log_file : & mut File , executor : Arc < dyn Execv + Send + Sync > ,) -> anyhow :: Result < () > ; }
    };
}

CargoCommand!();