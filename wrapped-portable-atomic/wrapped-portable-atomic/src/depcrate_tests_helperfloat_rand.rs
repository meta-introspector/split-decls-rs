// Generated macro for float_rand (module)
macro_rules! Depcrate_tests_helperfloat_rand {
() => {
// Module: crate::tests::helper
// Provides: {"float_rand"}
// Dependencies: {}
# [cfg (feature = "float")] pub (crate) mod float_rand { # [cfg (portable_atomic_unstable_f16)] pub (crate) fn f16 (rng : & mut fastrand :: Rng) -> f16 { f16 :: from_bits (rng . u16 (..)) } pub (crate) fn f32 (rng : & mut fastrand :: Rng) -> f32 { f32 :: from_bits (rng . u32 (..)) } pub (crate) fn f64 (rng : & mut fastrand :: Rng) -> f64 { f64 :: from_bits (rng . u64 (..)) } # [cfg (portable_atomic_unstable_f128)] pub (crate) fn f128 (rng : & mut fastrand :: Rng) -> f128 { f128 :: from_bits (rng . u128 (..)) } }
};
}
