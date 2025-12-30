// Generated macro for impl_716 (impl)
macro_rules! Depcrate_helpersimpl_716 {
() => {
// Module: crate::helpers
// Provides: {"impl_716"}
// Dependencies: {}
impl ToSoft for f64 { type SoftFloat = rustc_apfloat :: ieee :: Double ; fn to_soft (self) -> Self :: SoftFloat { Float :: from_bits (self . to_bits () . into ()) } }
};
}
