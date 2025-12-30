// Generated macro for impl_82 (impl)
macro_rules! Depcrate_errorimpl_82 {
() => {
// Module: crate::error
// Provides: {"impl_82"}
// Dependencies: {}
impl From < alloc :: string :: FromUtf8Error > for Error { fn from (_ : alloc :: string :: FromUtf8Error) -> Self { WIN32_ERROR (ERROR_NO_UNICODE_TRANSLATION) . into () } }
};
}
