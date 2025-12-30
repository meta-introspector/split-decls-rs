// Generated macro for impl_447 (impl)
macro_rules! Depcrate_uintimpl_447 {
() => {
// Module: crate::uint
// Provides: {"impl_447"}
// Dependencies: {}
impl < const LIMBS : usize > fmt :: Debug for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Uint(0x{:X})" , self . as_uint_ref ()) } }
};
}
