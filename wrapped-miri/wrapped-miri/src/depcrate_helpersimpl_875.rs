// Generated macro for impl_875 (impl)
macro_rules! Depcrate_helpersimpl_875 {
() => {
// Module: crate::helpers
// Provides: {"impl_875"}
// Dependencies: {}
impl ToSoft for f64 { type SoftFloat = rustc_apfloat :: ieee :: Double ; fn to_soft (self) -> Self :: SoftFloat { Float :: from_bits (self . to_bits () . into ()) } }
};
}
