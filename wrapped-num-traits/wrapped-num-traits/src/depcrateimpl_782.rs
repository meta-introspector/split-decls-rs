// Generated macro for impl_782 (impl)
macro_rules! Depcrateimpl_782 {
() => {
// Module: crate
// Provides: {"impl_782"}
// Dependencies: {}
impl fmt :: Display for ParseFloatError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let description = match self . kind { FloatErrorKind :: Empty => "cannot parse float from empty string" , FloatErrorKind :: Invalid => "invalid float literal" , } ; description . fmt (f) } }
};
}
