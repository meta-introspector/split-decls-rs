// Generated macro for impl_617 (impl)
macro_rules! Depcrate_errorimpl_617 {
() => {
// Module: crate::error
// Provides: {"impl_617"}
// Dependencies: {}
impl From < header :: InvalidHeaderName > for Error { fn from (err : header :: InvalidHeaderName) -> Error { Error { inner : ErrorKind :: HeaderName (err) , } } }
};
}
