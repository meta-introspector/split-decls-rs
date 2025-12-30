// Generated macro for remove_dir_all (function)
macro_rules! Depcrate_fsremove_dir_all {
() => {
// Module: crate::fs
// Provides: {"remove_dir_all"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::remove_dir_all`] which includes the file path in the panic message."] # [track_caller] pub fn remove_dir_all < P : AsRef < Path > > (path : P) { std :: fs :: remove_dir_all (path . as_ref ()) . expect (& format ! ("the directory in path \"{}\" could not be removed alongside all its contents" , path . as_ref () . display () ,)) ; }
};
}
