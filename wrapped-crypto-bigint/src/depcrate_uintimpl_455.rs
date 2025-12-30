// Generated macro for impl_455 (impl)
macro_rules! Depcrate_uintimpl_455 {
() => {
// Module: crate::uint
// Provides: {"impl_455"}
// Dependencies: {}
impl < const LIMBS : usize > fmt :: Debug for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Uint(0x{:X})" , self . as_uint_ref ()) } }
};
}
