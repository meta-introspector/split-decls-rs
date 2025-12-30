// Generated macro for impl_67 (impl)
macro_rules! Depcrateimpl_67 {
() => {
// Module: crate
// Provides: {"impl_67"}
// Dependencies: {}
impl fmt :: Display for BigInt { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (abs , is_neg) = self . abs () ; f . pad_integral (! is_neg , "" , & abs . to_string_unchecked (10)) } }
};
}
