// Generated macro for impl_154 (impl)
macro_rules! Depcrate_errorimpl_154 {
() => {
// Module: crate::error
// Provides: {"impl_154"}
// Dependencies: {}
impl From < Utf8Error > for Error { fn from (err : Utf8Error) -> Error { Error { kind : ErrorKind :: Utf8 (err) , position : None , } } }
};
}
