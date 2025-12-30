// Generated macro for impl_459 (impl)
macro_rules! Depcrate_uintimpl_459 {
() => {
// Module: crate::uint
// Provides: {"impl_459"}
// Dependencies: {}
impl < const LIMBS : usize > fmt :: UpperHex for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (self . as_uint_ref () , f) } }
};
}
