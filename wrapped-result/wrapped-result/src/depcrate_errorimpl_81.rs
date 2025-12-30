// Generated macro for impl_81 (impl)
macro_rules! Depcrate_errorimpl_81 {
() => {
// Module: crate::error
// Provides: {"impl_81"}
// Dependencies: {}
impl From < alloc :: string :: FromUtf16Error > for Error { fn from (_ : alloc :: string :: FromUtf16Error) -> Self { WIN32_ERROR (ERROR_NO_UNICODE_TRANSLATION) . into () } }
};
}
