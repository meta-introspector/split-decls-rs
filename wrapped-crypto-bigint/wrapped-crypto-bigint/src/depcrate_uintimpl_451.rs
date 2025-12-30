// Generated macro for impl_451 (impl)
macro_rules! Depcrate_uintimpl_451 {
() => {
// Module: crate::uint
// Provides: {"impl_451"}
// Dependencies: {}
impl < const LIMBS : usize > fmt :: UpperHex for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (self . as_uint_ref () , f) } }
};
}
