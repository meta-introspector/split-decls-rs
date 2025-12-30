// Generated macro for impl_715 (impl)
macro_rules! Depcrate_helpersimpl_715 {
() => {
// Module: crate::helpers
// Provides: {"impl_715"}
// Dependencies: {}
impl ToHost for rustc_apfloat :: ieee :: Double { type HostFloat = f64 ; fn to_host (self) -> Self :: HostFloat { f64 :: from_bits (self . to_bits () . try_into () . unwrap ()) } }
};
}
