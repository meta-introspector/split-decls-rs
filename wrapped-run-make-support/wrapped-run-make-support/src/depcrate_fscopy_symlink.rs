// Generated macro for copy_symlink (function)
macro_rules! Depcrate_fscopy_symlink {
() => {
// Module: crate::fs
// Provides: {"copy_symlink"}
// Dependencies: {}
# [doc = " Given a symlink at `src`, read its target, then create a new symlink at `dst` also pointing to"] # [doc = " target."] pub fn copy_symlink (src : impl AsRef < Path > , dst : impl AsRef < Path >) { let src = src . as_ref () ; let dst = dst . as_ref () ; let metadata = symlink_metadata (src) ; if let Err (e) = copy_symlink_raw (metadata . file_type () , src , dst) { panic ! ("failed to copy symlink from `{}` to `{}`: {e}" , src . display () , dst . display () ,) ; } }
};
}
