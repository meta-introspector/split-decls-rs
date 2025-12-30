// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [derive (Debug)] enum ErrorKind { Unsupported { actual : & 'static str , expected : & 'static str , } , InvalidValue { reason : & 'static str , } , OutsideContainer { method : & 'static str , } , # [allow (dead_code)] NoAlloc { method : & 'static str , } , }
};
}
