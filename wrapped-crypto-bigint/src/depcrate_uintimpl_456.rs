// Generated macro for impl_456 (impl)
macro_rules! Depcrate_uintimpl_456 {
() => {
// Module: crate::uint
// Provides: {"impl_456"}
// Dependencies: {}
impl < const LIMBS : usize > fmt :: Binary for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Binary :: fmt (self . as_uint_ref () , f) } }
};
}
