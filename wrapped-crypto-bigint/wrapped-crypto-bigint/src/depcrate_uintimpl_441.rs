// Generated macro for impl_441 (impl)
macro_rules! Depcrate_uintimpl_441 {
() => {
// Module: crate::uint
// Provides: {"impl_441"}
// Dependencies: {}
impl < const LIMBS : usize > ConditionallySelectable for Uint < LIMBS > { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { let mut limbs = [Limb :: ZERO ; LIMBS] ; for i in 0 .. LIMBS { limbs [i] = Limb :: conditional_select (& a . limbs [i] , & b . limbs [i] , choice) ; } Self { limbs } } }
};
}
