// Generated macro for create_dir (function)
macro_rules! Depcrate_fscreate_dir {
() => {
// Module: crate::fs
// Provides: {"create_dir"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::create_dir`] which includes the file path in the panic message."] # [track_caller] pub fn create_dir < P : AsRef < Path > > (path : P) { std :: fs :: create_dir (path . as_ref ()) . expect (& format ! ("the directory in path \"{}\" could not be created" , path . as_ref () . display ())) ; }
};
}
