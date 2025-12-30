// Generated macro for impl_10 (impl)
macro_rules! Depcrate_errorimpl_10 {
() => {
// Module: crate::error
// Provides: {"impl_10"}
// Dependencies: {}
impl fmt :: Display for ExtractError < Request > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ExtractError :: MethodMismatch (req) => { write ! (f , "Method mismatch for request '{}'" , req . method) } ExtractError :: JsonError { method , error } => { write ! (f , "Invalid request\nMethod: {method}\n error: {error}" ,) } } } }
};
}
