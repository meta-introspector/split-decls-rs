// Generated macro for impl_184 (impl)
macro_rules! Depcrate_castimpl_184 {
() => {
// Module: crate::cast
// Provides: {"impl_184"}
// Dependencies: {}
impl < T : NumCast > NumCast for Wrapping < T > { fn from < U : ToPrimitive > (n : U) -> Option < Self > { T :: from (n) . map (Wrapping) } }
};
}
