// Generated macro for read_to_string (function)
macro_rules! Depcrate_fsread_to_string {
() => {
// Module: crate::fs
// Provides: {"read_to_string"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::read_to_string`] which includes the file path in the panic message."] # [track_caller] pub fn read_to_string < P : AsRef < Path > > (path : P) -> String { std :: fs :: read_to_string (path . as_ref ()) . expect (& format ! ("the file in path \"{}\" could not be read into a String" , path . as_ref () . display ())) }
};
}
