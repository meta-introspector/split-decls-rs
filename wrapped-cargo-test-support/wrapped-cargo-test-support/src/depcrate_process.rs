// Generated macro for _process (function)
macro_rules! Depcrate_process {
() => {
// Module: crate
// Provides: {"_process"}
// Dependencies: {}
fn _process (t : & OsStr) -> ProcessBuilder { let mut p = ProcessBuilder :: new (t) ; p . cwd (& paths :: root ()) . test_env () ; p }
};
}
