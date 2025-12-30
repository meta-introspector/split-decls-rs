// Generated macro for read_dir (function)
macro_rules! Depcrate_fsread_dir {
() => {
// Module: crate::fs
// Provides: {"read_dir"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::read_dir`] which includes the file path in the panic message."] # [track_caller] pub fn read_dir < P : AsRef < Path > > (path : P) -> std :: fs :: ReadDir { std :: fs :: read_dir (path . as_ref ()) . expect (& format ! ("the directory in path \"{}\" could not be read" , path . as_ref () . display ())) }
};
}
