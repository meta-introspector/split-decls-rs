// Generated macro for run_clang (function)
macro_rules! Depcrate_supportrun_clang {
() => {
// Module: crate::support
// Provides: {"run_clang"}
// Dependencies: {}
# [doc = " Runs `clang`, returning the `stdout` and `stderr` output."] fn run_clang (path : & Path , arguments : & [& str]) -> (String , String) { run (& path . to_string_lossy () , arguments) . unwrap () }
};
}
