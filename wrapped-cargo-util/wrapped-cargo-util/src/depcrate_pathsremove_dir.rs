// Generated macro for remove_dir (function)
macro_rules! Depcrate_pathsremove_dir {
() => {
// Module: crate::paths
// Provides: {"remove_dir"}
// Dependencies: {}
# [doc = " Equivalent to [`std::fs::remove_dir`] with better error messages."] pub fn remove_dir < P : AsRef < Path > > (p : P) -> Result < () > { _remove_dir (p . as_ref ()) }
};
}
