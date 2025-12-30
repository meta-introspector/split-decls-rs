// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl fmt :: Display for ParseIntError { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . kind { IntErrorKind :: Empty => "cannot parse integer from empty string" , IntErrorKind :: InvalidDigit => "invalid digit found in string" , IntErrorKind :: PosOverflow => "number too large to fit in target type" , IntErrorKind :: NegOverflow => "number too small to fit in target type" , IntErrorKind :: Zero => "number would be zero for non-zero type" , _ => "Unknown Int error kind" , } . fmt (f) } }
};
}
