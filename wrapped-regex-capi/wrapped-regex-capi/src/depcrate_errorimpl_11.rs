// Generated macro for impl_11 (impl)
macro_rules! Depcrate_errorimpl_11 {
() => {
// Module: crate::error
// Provides: {"impl_11"}
// Dependencies: {}
impl Error { pub fn new (kind : ErrorKind) -> Error { Error { message : None , kind } } pub fn is_err (& self) -> bool { match self . kind { ErrorKind :: None => false , ErrorKind :: Str (_) | ErrorKind :: Regex (_) | ErrorKind :: Nul (_) => { true } } } }
};
}
