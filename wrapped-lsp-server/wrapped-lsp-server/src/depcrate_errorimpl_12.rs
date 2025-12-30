// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorimpl_12 {
() => {
// Module: crate::error
// Provides: {"impl_12"}
// Dependencies: {}
impl fmt :: Display for ExtractError < Notification > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ExtractError :: MethodMismatch (req) => { write ! (f , "Method mismatch for notification '{}'" , req . method) } ExtractError :: JsonError { method , error } => { write ! (f , "Invalid notification\nMethod: {method}\n error: {error}") } } } }
};
}
