// Generated macro for impl_613 (impl)
macro_rules! Depcrate_errorimpl_613 {
() => {
// Module: crate::error
// Provides: {"impl_613"}
// Dependencies: {}
impl From < status :: InvalidStatusCode > for Error { fn from (err : status :: InvalidStatusCode) -> Error { Error { inner : ErrorKind :: StatusCode (err) , } } }
};
}
