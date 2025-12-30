// Generated macro for impl_874 (impl)
macro_rules! Depcrate_helpersimpl_874 {
() => {
// Module: crate::helpers
// Provides: {"impl_874"}
// Dependencies: {}
impl ToHost for rustc_apfloat :: ieee :: Double { type HostFloat = f64 ; fn to_host (self) -> Self :: HostFloat { f64 :: from_bits (self . to_bits () . try_into () . unwrap ()) } }
};
}
