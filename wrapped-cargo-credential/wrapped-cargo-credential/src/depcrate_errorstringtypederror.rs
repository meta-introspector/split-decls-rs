// Generated macro for StringTypedError (struct)
macro_rules! Depcrate_errorStringTypedError {
() => {
// Module: crate::error
// Provides: {"StringTypedError"}
// Dependencies: {}
# [doc = " String-based error type with an optional source"] # [derive (Debug)] struct StringTypedError { message : String , source : Option < Box < StringTypedError > > , }
};
}
