// Generated macro for impl_445 (impl)
macro_rules! Depcrate_uintimpl_445 {
() => {
// Module: crate::uint
// Provides: {"impl_445"}
// Dependencies: {}
impl < const LIMBS : usize > num_traits :: Zero for Uint < LIMBS > { # [inline (always)] fn zero () -> Self { Self :: ZERO } fn is_zero (& self) -> bool { self . ct_eq (& Self :: ZERO) . into () } }
};
}
