// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
impl fmt :: Octal for BigInt { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (abs , is_neg) = self . abs () ; f . pad_integral (! is_neg , "0o" , & abs . to_string_unchecked (8)) } }
};
}
