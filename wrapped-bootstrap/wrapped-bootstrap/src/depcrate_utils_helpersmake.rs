// Generated macro for make (function)
macro_rules! Depcrate_utils_helpersmake {
() => {
// Module: crate::utils::helpers
// Provides: {"make"}
// Dependencies: {}
pub fn make (host : & str) -> PathBuf { if host . contains ("dragonfly") || host . contains ("freebsd") || host . contains ("netbsd") || host . contains ("openbsd") { PathBuf :: from ("gmake") } else { PathBuf :: from ("make") } }
};
}
