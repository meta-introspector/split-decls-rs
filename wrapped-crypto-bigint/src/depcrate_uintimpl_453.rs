// Generated macro for impl_453 (impl)
macro_rules! Depcrate_uintimpl_453 {
() => {
// Module: crate::uint
// Provides: {"impl_453"}
// Dependencies: {}
impl < const LIMBS : usize > num_traits :: Zero for Uint < LIMBS > { # [inline (always)] fn zero () -> Self { Self :: ZERO } fn is_zero (& self) -> bool { self . ct_eq (& Self :: ZERO) . into () } }
};
}
