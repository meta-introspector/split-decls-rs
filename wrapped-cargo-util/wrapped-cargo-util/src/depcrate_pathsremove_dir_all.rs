// Generated macro for remove_dir_all (function)
macro_rules! Depcrate_pathsremove_dir_all {
() => {
// Module: crate::paths
// Provides: {"remove_dir_all"}
// Dependencies: {}
# [doc = " Equivalent to [`std::fs::remove_dir_all`] with better error messages."] # [doc = ""] # [doc = " This does *not* follow symlinks."] pub fn remove_dir_all < P : AsRef < Path > > (p : P) -> Result < () > { _remove_dir_all (p . as_ref ()) . or_else (| prev_err | { fs :: remove_dir_all (p . as_ref ()) . with_context (| | { format ! ("{:?}\n\nError: failed to remove directory `{}`" , prev_err , p . as_ref () . display () ,) }) }) }
};
}
