// Generated macro for testdir (function)
macro_rules! Depcrate_core_build_steps_testtestdir {
() => {
// Module: crate::core::build_steps::test
// Provides: {"testdir"}
// Dependencies: {}
fn testdir (builder : & Builder < '_ > , host : TargetSelection) -> PathBuf { builder . out . join (host) . join ("test") }
};
}
