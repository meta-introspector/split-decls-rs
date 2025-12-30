// Generated macro for create_dir_all (function)
macro_rules! Depcrate_pathscreate_dir_all {
() => {
// Module: crate::paths
// Provides: {"create_dir_all"}
// Dependencies: {}
# [doc = " Equivalent to [`std::fs::create_dir_all`] with better error messages."] pub fn create_dir_all (p : impl AsRef < Path >) -> Result < () > { _create_dir_all (p . as_ref ()) }
};
}
