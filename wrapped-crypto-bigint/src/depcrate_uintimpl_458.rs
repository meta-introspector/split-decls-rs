// Generated macro for impl_458 (impl)
macro_rules! Depcrate_uintimpl_458 {
() => {
// Module: crate::uint
// Provides: {"impl_458"}
// Dependencies: {}
impl < const LIMBS : usize > fmt :: LowerHex for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (self . as_uint_ref () , f) } }
};
}
