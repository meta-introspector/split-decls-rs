macro_rules! deps {
    () => {
        RealFileSystemStat!();
    };
}

macro_rules! run_collect_repo_state_command {
    () => {
        deps!();
        pub fn run_collect_repo_state_command (project_root : PathBuf) -> Result < () > { println ! ("Collecting repository state for: {:?}" , project_root) ; let git_executable_path = PathBuf :: from ("git") ; let executor = Arc :: new (RealExecv { }) ; let file_system_stat = Arc :: new (RealFileSystemStat { }) ; let rollup_lock = Arc :: new (Mutex :: new (RollupLock :: load (& project_root) ?)) ; let git_executor : Arc < dyn GitExecutor + Send + Sync > = { # [cfg (feature = "git_enabled")] { Arc :: new (PureRustGitExecutor :: new (rollup_lock . clone () , project_root . clone () ,)) } # [cfg (not (feature = "git_enabled"))] { Arc :: new (DummyGitExecutor) } } ; Ok (()) }
    };
}

run_collect_repo_state_command!();