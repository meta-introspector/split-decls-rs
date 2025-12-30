// Generated macro for impl_61 (impl)
macro_rules! Depcrate_errorimpl_61 {
() => {
// Module: crate::error
// Provides: {"impl_61"}
// Dependencies: {}
impl Error { pub fn new (kind : ErrorKind) -> Error { Error { message : None , kind : kind , } } pub fn is_err (& self) -> bool { match self . kind { ErrorKind :: None => false , ErrorKind :: Str (_) | ErrorKind :: Regex (_) => true , } } }
};
}
