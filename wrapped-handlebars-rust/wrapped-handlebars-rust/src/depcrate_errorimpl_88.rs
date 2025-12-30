// Generated macro for impl_88 (impl)
macro_rules! Depcrate_errorimpl_88 {
() => {
// Module: crate::error
// Provides: {"impl_88"}
// Dependencies: {}
impl From < IOError > for RenderError { fn from (e : IOError) -> RenderError { RenderErrorReason :: IOError (e) . into () } }
};
}
