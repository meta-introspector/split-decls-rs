// Generated macro for impl_290 (impl)
macro_rules! Depcrate_oddimpl_290 {
() => {
// Module: crate::odd
// Provides: {"impl_290"}
// Dependencies: {}
impl < const LIMBS : usize > PartialOrd < Odd < Uint < LIMBS > > > for Uint < LIMBS > { fn partial_cmp (& self , other : & Odd < Uint < LIMBS > >) -> Option < Ordering > { Some (self . cmp (& other . 0)) } }
};
}
