// Generated macro for impl_99 (impl)
macro_rules! Depcrate_hresultimpl_99 {
() => {
// Module: crate::hresult
// Provides: {"impl_99"}
// Dependencies: {}
impl < T > From < Result < T > > for HRESULT { fn from (result : Result < T >) -> Self { if let Err (error) = result { return error . into () ; } Self (0) } }
};
}
