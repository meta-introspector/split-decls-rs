// Generated macro for impl_446 (impl)
macro_rules! Depcrate_uintimpl_446 {
() => {
// Module: crate::uint
// Provides: {"impl_446"}
// Dependencies: {}
impl < const LIMBS : usize > Integer for Uint < LIMBS > { fn as_limbs (& self) -> & [Limb] { & self . limbs } fn as_mut_limbs (& mut self) -> & mut [Limb] { & mut self . limbs } fn nlimbs (& self) -> usize { Self :: LIMBS } }
};
}
