// Generated macro for dir_is_empty (function)
macro_rules! Depcrate_utils_helpersdir_is_empty {
() => {
// Module: crate::utils::helpers
// Provides: {"dir_is_empty"}
// Dependencies: {}
pub fn dir_is_empty (dir : & Path) -> bool { t ! (std :: fs :: read_dir (dir) , dir) . next () . is_none () }
};
}
