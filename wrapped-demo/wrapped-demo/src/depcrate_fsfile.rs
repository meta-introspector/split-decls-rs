// Generated macro for file (function)
macro_rules! Depcrate_fsfile {
() => {
// Module: crate::fs
// Provides: {"file"}
// Dependencies: {}
pub fn file () -> io :: Result < () > { eprintln ! () ; test_file ("/tmp/hello.txt") ? ; if cfg ! (target_os = "hermit") && cfg ! (feature = "fs") { test_file ("/root/hello.txt") ? ; } Ok (()) }
};
}
