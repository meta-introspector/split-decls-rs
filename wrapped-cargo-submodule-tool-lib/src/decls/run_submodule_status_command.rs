macro_rules! run_submodule_status_command {
    () => {
        pub fn run_submodule_status_command (args : & SubmoduleStatusArgs) -> Result < () > { let root_dir = args . root_dir . canonicalize () . context ("Failed to canonicalize root_dir") ? ; let git_executable_path = PathBuf :: from ("git") ; let executor = Arc :: new (RealExecv { }) ; let rollup_lock_arc = Arc :: new (Mutex :: new (RollupLock :: load (& root_dir) ?)) ; let git_executor : Arc < dyn GitExecutor + Send + Sync > = { # [cfg (feature = "git_enabled")] { Arc :: new (PureRustGitExecutor :: new (rollup_lock_arc . clone () , root_dir . clone () ,)) } # [cfg (not (feature = "git_enabled"))] { Arc :: new (DummyGitExecutor) } } ; let status_output = git_executor . status (& root_dir) ? ; println ! ("{}" , status_output) ; Ok (()) }
    };
}

run_submodule_status_command!()