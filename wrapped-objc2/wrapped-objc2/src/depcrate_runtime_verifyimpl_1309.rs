// Generated macro for impl_1309 (impl)
macro_rules! Depcrate_runtime_verifyimpl_1309 {
() => {
// Module: crate::runtime::verify
// Provides: {"impl_1309"}
// Dependencies: {}
impl fmt :: Display for Inner { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: MethodNotFound => write ! (f , "method not found") , Self :: EncodingParseError (e) => write ! (f , "{e}") , Self :: MismatchedReturn (expected , actual) => { write ! (f , "expected return to have type code '{expected}', but found '{actual}'" ,) } Self :: MismatchedArgumentsCount (expected , actual) => { write ! (f , "expected {expected} arguments, but {actual} were given" ,) } Self :: MismatchedArgument (i , expected , actual) => { write ! (f , "expected argument at index {i} to have type code '{expected}', but found '{actual}'" ,) } } } }
};
}
