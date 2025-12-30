// Generated macro for impl_454 (impl)
macro_rules! Depcrate_uintimpl_454 {
() => {
// Module: crate::uint
// Provides: {"impl_454"}
// Dependencies: {}
impl < const LIMBS : usize > num_traits :: One for Uint < LIMBS > { # [inline (always)] fn one () -> Self { Self :: ONE } fn is_one (& self) -> bool { self . ct_eq (& Self :: ONE) . into () } }
};
}
