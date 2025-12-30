// Generated macro for impl_78 (impl)
macro_rules! Depcrate_errorimpl_78 {
() => {
// Module: crate::error
// Provides: {"impl_78"}
// Dependencies: {}
impl From < HRESULT > for Error { fn from (code : HRESULT) -> Self { Self { code : nonzero_hresult (code) , info : ErrorInfo :: from_thread () , } } }
};
}
