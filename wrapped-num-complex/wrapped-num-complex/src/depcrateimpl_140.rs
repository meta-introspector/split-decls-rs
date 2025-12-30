// Generated macro for impl_140 (impl)
macro_rules! Depcrateimpl_140 {
() => {
// Module: crate
// Provides: {"impl_140"}
// Dependencies: {}
impl < E : fmt :: Display > fmt :: Display for ParseComplexError < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . kind { ComplexErrorKind :: ParseError (ref e) => e . fmt (f) , ComplexErrorKind :: ExprError => "invalid or unsupported complex expression" . fmt (f) , ComplexErrorKind :: UnsupportedRadix => "unsupported radix for conversion" . fmt (f) , } } }
};
}
