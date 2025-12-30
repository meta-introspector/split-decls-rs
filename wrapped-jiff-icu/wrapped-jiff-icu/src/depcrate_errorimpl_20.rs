// Generated macro for impl_20 (impl)
macro_rules! Depcrate_errorimpl_20 {
() => {
// Module: crate::error
// Provides: {"impl_20"}
// Dependencies: {}
impl From < jiff :: Error > for Error { fn from (e : jiff :: Error) -> Error { Error { kind : ErrorKind :: Jiff (e) } } }
};
}
