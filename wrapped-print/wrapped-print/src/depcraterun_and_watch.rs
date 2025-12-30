// Generated macro for run_and_watch (function)
macro_rules! Depcraterun_and_watch {
() => {
// Module: crate
// Provides: {"run_and_watch"}
// Dependencies: {}
async fn run_and_watch (opts : Opts , source : & mut Source) -> anyhow :: Result < () > { let (tx , mut rx) = tokio :: sync :: mpsc :: channel (1) ; let path = opts . elf . clone () . unwrap () . canonicalize () . unwrap () ; let directory_path = path . parent () . unwrap () ; let mut watcher = RecommendedWatcher :: new (move | res | { let _ = tx . blocking_send (res) ; } , Config :: default () ,) ? ; watcher . watch (directory_path . as_ref () , RecursiveMode :: NonRecursive) ? ; loop { select ! { r = run (opts . clone () , source) => r ?, _ = has_file_changed (& mut rx , & path) => () } } }
};
}
