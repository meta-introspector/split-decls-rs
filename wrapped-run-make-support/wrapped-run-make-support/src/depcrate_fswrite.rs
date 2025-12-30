// Generated macro for write (function)
macro_rules! Depcrate_fswrite {
() => {
// Module: crate::fs
// Provides: {"write"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::write`] which includes the file path in the panic message."] # [track_caller] pub fn write < P : AsRef < Path > , C : AsRef < [u8] > > (path : P , contents : C) { std :: fs :: write (path . as_ref () , contents . as_ref ()) . expect (& format ! ("the file in path \"{}\" could not be written to" , path . as_ref () . display ())) ; }
};
}
