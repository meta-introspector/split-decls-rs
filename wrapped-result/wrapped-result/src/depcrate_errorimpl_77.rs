// Generated macro for impl_77 (impl)
macro_rules! Depcrate_errorimpl_77 {
() => {
// Module: crate::error
// Provides: {"impl_77"}
// Dependencies: {}
impl From < Error > for HRESULT { fn from (error : Error) -> Self { let code = error . code () ; error . info . into_thread () ; code } }
};
}
