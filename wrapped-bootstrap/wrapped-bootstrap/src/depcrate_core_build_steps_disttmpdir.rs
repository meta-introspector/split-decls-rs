// Generated macro for tmpdir (function)
macro_rules! Depcrate_core_build_steps_disttmpdir {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"tmpdir"}
// Dependencies: {}
pub fn tmpdir (builder : & Builder < '_ >) -> PathBuf { builder . out . join ("tmp/dist") }
};
}
