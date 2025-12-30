// Generated macro for impl_31 (impl)
macro_rules! Depcrate_errorimpl_31 {
() => {
// Module: crate::error
// Provides: {"impl_31"}
// Dependencies: {}
impl From < ffi :: NulError > for Error { fn from (_ : ffi :: NulError) -> Error { Error { code : curl_sys :: CURLE_CONV_FAILED , extra : None , } } }
};
}
