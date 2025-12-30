// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl fmt :: LowerHex for BigInt { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (abs , is_neg) = self . abs () ; f . pad_integral (! is_neg , "0x" , & abs . to_string_unchecked (16)) } }
};
}
