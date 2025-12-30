// Generated macro for impl_450 (impl)
macro_rules! Depcrate_uintimpl_450 {
() => {
// Module: crate::uint
// Provides: {"impl_450"}
// Dependencies: {}
impl < const LIMBS : usize > fmt :: LowerHex for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (self . as_uint_ref () , f) } }
};
}
