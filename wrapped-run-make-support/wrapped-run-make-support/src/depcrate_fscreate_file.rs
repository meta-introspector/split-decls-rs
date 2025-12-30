// Generated macro for create_file (function)
macro_rules! Depcrate_fscreate_file {
() => {
// Module: crate::fs
// Provides: {"create_file"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::File::create`] which includes the file path in the panic message."] # [track_caller] pub fn create_file < P : AsRef < Path > > (path : P) { std :: fs :: File :: create (path . as_ref ()) . expect (& format ! ("the file in path \"{}\" could not be created" , path . as_ref () . display ())) ; }
};
}
