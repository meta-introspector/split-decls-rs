// Generated macro for impl_615 (impl)
macro_rules! Depcrate_errorimpl_615 {
() => {
// Module: crate::error
// Provides: {"impl_615"}
// Dependencies: {}
impl From < uri :: InvalidUri > for Error { fn from (err : uri :: InvalidUri) -> Error { Error { inner : ErrorKind :: Uri (err) , } } }
};
}
