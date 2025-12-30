// Generated macro for recursive_remove (function)
macro_rules! Depcrate_fsrecursive_remove {
() => {
// Module: crate::fs
// Provides: {"recursive_remove"}
// Dependencies: {}
# [doc = " A wrapper around [`build_helper::fs::recursive_remove`] which includes the file path in the"] # [doc = " panic message."] # [doc = ""] # [doc = " This handles removing symlinks on Windows (e.g. symlink-to-file will be removed via"] # [doc = " [`std::fs::remove_file`] while symlink-to-dir will be removed via [`std::fs::remove_dir`])."] # [track_caller] pub fn recursive_remove < P : AsRef < Path > > (path : P) { if let Err (e) = build_helper :: fs :: recursive_remove (path . as_ref ()) { panic ! ("failed to recursive remove filesystem entities at `{}`: {e}" , path . as_ref () . display ()) ; } }
};
}
