// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl fmt :: UpperHex for BigInt { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (abs , is_neg) = self . abs () ; let mut s : String = abs . to_string_unchecked (16) ; s . make_ascii_uppercase () ; f . pad_integral (! is_neg , "0x" , & s) } }
};
}
