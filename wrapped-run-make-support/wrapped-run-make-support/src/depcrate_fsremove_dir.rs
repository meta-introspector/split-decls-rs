// Generated macro for remove_dir (function)
macro_rules! Depcrate_fsremove_dir {
() => {
// Module: crate::fs
// Provides: {"remove_dir"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::remove_dir`] which includes the directory path in the panic message."] # [track_caller] pub fn remove_dir < P : AsRef < Path > > (path : P) { if let Err (e) = std :: fs :: remove_dir (path . as_ref ()) { panic ! ("failed to remove directory at `{}`: {e}" , path . as_ref () . display ()) ; } }
};
}
