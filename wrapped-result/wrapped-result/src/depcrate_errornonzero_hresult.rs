// Generated macro for nonzero_hresult (function)
macro_rules! Depcrate_errornonzero_hresult {
() => {
// Module: crate::error
// Provides: {"nonzero_hresult"}
// Dependencies: {}
fn nonzero_hresult (hr : HRESULT) -> NonZeroI32 { if let Some (nz) = NonZeroI32 :: new (hr . 0) { nz } else { S_EMPTY_ERROR } }
};
}
