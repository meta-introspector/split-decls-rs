// Generated macro for impl_614 (impl)
macro_rules! Depcrate_errorimpl_614 {
() => {
// Module: crate::error
// Provides: {"impl_614"}
// Dependencies: {}
impl From < method :: InvalidMethod > for Error { fn from (err : method :: InvalidMethod) -> Error { Error { inner : ErrorKind :: Method (err) , } } }
};
}
