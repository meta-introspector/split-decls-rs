macro_rules! deps {
    () => {
        CachedFileSystemWriter!();
        Cli!();
        FileSystemWriter!();
        RealFileSystemWriter!();
        RealFileSystemStat!();
    };
}

macro_rules! run_add_submodules_command {
    () => {
        deps!();
        pub fn run_add_submodules_command (args : & AddSubmodulesArgs , cli : & Cli) -> Result < () > { println ! ("Collecting repository state for: {:?}" , args . root_dir) ; let git_executable_path = PathBuf :: from ("git") ; let executor = Arc :: new (RealExecv { }) ; let rollup_lock_arc = Arc :: new (Mutex :: new (RollupLock :: load (& args . root_dir) ?)) ; let git_executor : Arc < dyn GitExecutor + Send + Sync > = { # [cfg (feature = "git_enabled")] { Arc :: new (PureRustGitExecutor :: new (rollup_lock_arc . clone () , args . root_dir . clone () ,)) } # [cfg (not (feature = "git_enabled"))] { Arc :: new (DummyGitExecutor) } } ; let real_file_system_stat = RealFileSystemStat :: new (git_executor . clone () , rollup_lock_arc . clone () , args . root_dir . clone () ,) ; let _file_system_writer : Box < dyn FileSystemWriter > = if cli . dry_run { Box :: new (CachedFileSystemWriter :: new (Arc :: new (real_file_system_stat . clone ()) , rollup_lock_arc . clone () , args . root_dir . clone () ,)) } else { Box :: new (RealFileSystemWriter) } ; Ok (()) }
    };
}

run_add_submodules_command!();