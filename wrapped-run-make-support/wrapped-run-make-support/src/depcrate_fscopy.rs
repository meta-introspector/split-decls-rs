// Generated macro for copy (function)
macro_rules! Depcrate_fscopy {
() => {
// Module: crate::fs
// Provides: {"copy"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::copy`] which includes the file path in the panic message."] # [track_caller] pub fn copy < P : AsRef < Path > , Q : AsRef < Path > > (from : P , to : Q) { std :: fs :: copy (from . as_ref () , to . as_ref ()) . expect (& format ! ("the file \"{}\" could not be copied over to \"{}\"" , from . as_ref () . display () , to . as_ref () . display () ,)) ; }
};
}
