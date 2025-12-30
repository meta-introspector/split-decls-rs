// Generated macro for impl_1025 (impl)
macro_rules! Depcrate_ssl_errorimpl_1025 {
() => {
// Module: crate::ssl::error
// Provides: {"impl_1025"}
// Dependencies: {}
impl From < ErrorStack > for Error { fn from (e : ErrorStack) -> Error { Error { code : ErrorCode :: SSL , cause : Some (InnerError :: Ssl (e)) , } } }
};
}
