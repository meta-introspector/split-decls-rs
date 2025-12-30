// Generated macro for impl_618 (impl)
macro_rules! Depcrate_errorimpl_618 {
() => {
// Module: crate::error
// Provides: {"impl_618"}
// Dependencies: {}
impl From < header :: InvalidHeaderValue > for Error { fn from (err : header :: InvalidHeaderValue) -> Error { Error { inner : ErrorKind :: HeaderValue (err) , } } }
};
}
