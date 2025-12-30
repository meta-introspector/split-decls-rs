// Generated macro for impl_58 (impl)
macro_rules! Depcrate_devimpl_58 {
() => {
// Module: crate::dev
// Provides: {"impl_58"}
// Dependencies: {}
# [cfg (feature = "bits")] impl PrimeFieldBits for Scalar { # [cfg (target_pointer_width = "32")] type ReprBits = [u32 ; 8] ; # [cfg (target_pointer_width = "64")] type ReprBits = [u64 ; 4] ; fn to_le_bits (& self) -> ScalarBits { self . 0 . as_uint () . to_words () . into () } fn char_le_bits () -> ScalarBits { MockCurve :: ORDER . to_words () . into () } }
};
}
