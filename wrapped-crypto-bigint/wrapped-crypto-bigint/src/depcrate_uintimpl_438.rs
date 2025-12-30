// Generated macro for impl_438 (impl)
macro_rules! Depcrate_uintimpl_438 {
() => {
// Module: crate::uint
// Provides: {"impl_438"}
// Dependencies: {}
impl < const LIMBS : usize > Integer for Uint < LIMBS > { fn as_limbs (& self) -> & [Limb] { & self . limbs } fn as_mut_limbs (& mut self) -> & mut [Limb] { & mut self . limbs } fn nlimbs (& self) -> usize { Self :: LIMBS } }
};
}
