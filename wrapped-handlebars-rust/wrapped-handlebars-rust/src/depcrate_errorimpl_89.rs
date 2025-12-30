// Generated macro for impl_89 (impl)
macro_rules! Depcrate_errorimpl_89 {
() => {
// Module: crate::error
// Provides: {"impl_89"}
// Dependencies: {}
impl From < FromUtf8Error > for RenderError { fn from (e : FromUtf8Error) -> Self { RenderErrorReason :: Utf8Error (e) . into () } }
};
}
