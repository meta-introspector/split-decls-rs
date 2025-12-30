// Generated macro for rename (function)
macro_rules! Depcrate_fsrename {
() => {
// Module: crate::fs
// Provides: {"rename"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::rename`] which includes the file path in the panic message."] # [track_caller] pub fn rename < P : AsRef < Path > , Q : AsRef < Path > > (from : P , to : Q) { std :: fs :: rename (from . as_ref () , to . as_ref ()) . expect (& format ! ("the file \"{}\" could not be moved over to \"{}\"" , from . as_ref () . display () , to . as_ref () . display () ,)) ; }
};
}
