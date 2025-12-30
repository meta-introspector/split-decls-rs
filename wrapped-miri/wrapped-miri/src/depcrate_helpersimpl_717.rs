// Generated macro for impl_717 (impl)
macro_rules! Depcrate_helpersimpl_717 {
() => {
// Module: crate::helpers
// Provides: {"impl_717"}
// Dependencies: {}
impl ToHost for rustc_apfloat :: ieee :: Single { type HostFloat = f32 ; fn to_host (self) -> Self :: HostFloat { f32 :: from_bits (self . to_bits () . try_into () . unwrap ()) } }
};
}
