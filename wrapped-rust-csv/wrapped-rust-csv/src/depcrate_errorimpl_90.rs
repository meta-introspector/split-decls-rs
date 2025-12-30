// Generated macro for impl_90 (impl)
macro_rules! Depcrate_errorimpl_90 {
() => {
// Module: crate::error
// Provides: {"impl_90"}
// Dependencies: {}
impl From < io :: Error > for Error { fn from (err : io :: Error) -> Error { Error :: new (ErrorKind :: Io (err)) } }
};
}
