// Generated macro for read (function)
macro_rules! Depcrate_fsread {
() => {
// Module: crate::fs
// Provides: {"read"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::read`] which includes the file path in the panic message."] # [track_caller] pub fn read < P : AsRef < Path > > (path : P) -> Vec < u8 > { std :: fs :: read (path . as_ref ()) . expect (& format ! ("the file in path \"{}\" could not be read" , path . as_ref () . display ())) }
};
}
