// Generated macro for open_file (function)
macro_rules! Depcrate_test_runner_replayopen_file {
() => {
// Module: crate::test_runner::replay
// Provides: {"open_file"}
// Dependencies: {}
# [doc = " Open the file in the usual read+append+create mode."] pub (crate) fn open_file (path : impl AsRef < Path >) -> io :: Result < fs :: File > { fs :: OpenOptions :: new () . read (true) . append (true) . create (true) . truncate (false) . open (path) }
};
}
