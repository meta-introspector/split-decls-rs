// Generated macro for impl_446 (impl)
macro_rules! Depcrate_uintimpl_446 {
() => {
// Module: crate::uint
// Provides: {"impl_446"}
// Dependencies: {}
impl < const LIMBS : usize > num_traits :: One for Uint < LIMBS > { # [inline (always)] fn one () -> Self { Self :: ONE } fn is_one (& self) -> bool { self . ct_eq (& Self :: ONE) . into () } }
};
}
