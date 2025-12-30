// Generated macro for project_root (function)
macro_rules! Depcrate_testsproject_root {
() => {
// Module: crate::tests
// Provides: {"project_root"}
// Dependencies: {}
pub fn project_root () -> PathBuf { let dir = env ! ("CARGO_MANIFEST_DIR") ; let mut res = PathBuf :: from (dir) ; while ! res . join ("README.md") . exists () { res = res . parent () . expect ("reached fs root without finding project root") . to_owned () } res }
};
}
