// Generated macro for impl_25 (impl)
macro_rules! Depcrate_btoiimpl_25 {
() => {
// Module: crate::btoi
// Provides: {"impl_25"}
// Dependencies: {}
impl ParseIntegerError { fn desc (& self) -> & str { match self . kind { ErrorKind :: Empty => "cannot parse integer without digits" , ErrorKind :: InvalidDigit => "invalid digit found in slice" , ErrorKind :: Overflow => "number too large to fit in target type" , ErrorKind :: Underflow => "number too small to fit in target type" , } } }
};
}
