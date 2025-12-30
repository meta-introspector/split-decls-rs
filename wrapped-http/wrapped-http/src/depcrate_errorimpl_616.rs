// Generated macro for impl_616 (impl)
macro_rules! Depcrate_errorimpl_616 {
() => {
// Module: crate::error
// Provides: {"impl_616"}
// Dependencies: {}
impl From < uri :: InvalidUriParts > for Error { fn from (err : uri :: InvalidUriParts) -> Error { Error { inner : ErrorKind :: UriParts (err) , } } }
};
}
