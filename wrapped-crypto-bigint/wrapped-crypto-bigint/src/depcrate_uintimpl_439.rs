// Generated macro for impl_439 (impl)
macro_rules! Depcrate_uintimpl_439 {
() => {
// Module: crate::uint
// Provides: {"impl_439"}
// Dependencies: {}
impl < const LIMBS : usize > Unsigned for Uint < LIMBS > { type Monty = MontyForm < LIMBS > ; fn from_limb_like (limb : Limb , _other : & Self) -> Self { Self :: from (limb) } }
};
}
