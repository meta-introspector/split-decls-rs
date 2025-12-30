// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl fmt :: Binary for BigInt { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (abs , is_neg) = self . abs () ; f . pad_integral (! is_neg , "0b" , & abs . to_string_unchecked (2)) } }
};
}
