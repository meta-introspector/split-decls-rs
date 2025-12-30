// Generated macro for impl_448 (impl)
macro_rules! Depcrate_uintimpl_448 {
() => {
// Module: crate::uint
// Provides: {"impl_448"}
// Dependencies: {}
impl < const LIMBS : usize > fmt :: Binary for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Binary :: fmt (self . as_uint_ref () , f) } }
};
}
