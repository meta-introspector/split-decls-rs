// Generated macro for remove_file (function)
macro_rules! Depcrate_fsremove_file {
() => {
// Module: crate::fs
// Provides: {"remove_file"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::remove_file`] which includes the file path in the panic message."] # [track_caller] pub fn remove_file < P : AsRef < Path > > (path : P) { if let Err (e) = std :: fs :: remove_file (path . as_ref ()) { panic ! ("failed to remove file at `{}`: {e}" , path . as_ref () . display ()) ; } }
};
}
