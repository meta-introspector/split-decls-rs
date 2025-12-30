// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum ErrorKind { Generic , # [cfg (feature = "std")] IO (std :: io :: Error) , InvalidKey , }
};
}
