// Generated macro for Inner (enum)
macro_rules! Depcrate_runtime_verifyInner {
() => {
// Module: crate::runtime::verify
// Provides: {"Inner"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Hash)] pub (crate) enum Inner { MethodNotFound , EncodingParseError (EncodingParseError) , MismatchedReturn (EncodingBox , Encoding) , MismatchedArgumentsCount (usize , usize) , MismatchedArgument (usize , EncodingBox , Encoding) , }
};
}
