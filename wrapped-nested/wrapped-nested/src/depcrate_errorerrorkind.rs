// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [derive (Debug)] enum ErrorKind { Buffer (sval_buffer :: Error) , InvalidValue { reason : & 'static str , } , # [cfg (not (feature = "alloc"))] # [allow (dead_code)] NoAlloc { method : & 'static str , } , }
};
}
