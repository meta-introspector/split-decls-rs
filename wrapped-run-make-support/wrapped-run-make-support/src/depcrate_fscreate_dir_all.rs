// Generated macro for create_dir_all (function)
macro_rules! Depcrate_fscreate_dir_all {
() => {
// Module: crate::fs
// Provides: {"create_dir_all"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::create_dir_all`] which includes the file path in the panic message."] # [track_caller] pub fn create_dir_all < P : AsRef < Path > > (path : P) { std :: fs :: create_dir_all (path . as_ref ()) . expect (& format ! ("the directory (and all its parents) in path \"{}\" could not be created" , path . as_ref () . display ())) ; }
};
}
