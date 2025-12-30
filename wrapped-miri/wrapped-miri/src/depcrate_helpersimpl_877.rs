// Generated macro for impl_877 (impl)
macro_rules! Depcrate_helpersimpl_877 {
() => {
// Module: crate::helpers
// Provides: {"impl_877"}
// Dependencies: {}
impl ToSoft for f32 { type SoftFloat = rustc_apfloat :: ieee :: Single ; fn to_soft (self) -> Self :: SoftFloat { Float :: from_bits (self . to_bits () . into ()) } }
};
}
