// Generated macro for impl_120 (impl)
macro_rules! Depcrate_win32_errorimpl_120 {
() => {
// Module: crate::win32_error
// Provides: {"impl_120"}
// Dependencies: {}
impl From < WIN32_ERROR > for Error { fn from (value : WIN32_ERROR) -> Self { value . to_hresult () . into () } }
};
}
