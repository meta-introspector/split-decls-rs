// Generated macro for impl_139 (impl)
macro_rules! Depcrateimpl_139 {
() => {
// Module: crate
// Provides: {"impl_139"}
// Dependencies: {}
# [cfg (feature = "std")] impl < E : Error > Error for ParseComplexError < E > { # [allow (deprecated)] fn description (& self) -> & str { match self . kind { ComplexErrorKind :: ParseError (ref e) => e . description () , ComplexErrorKind :: ExprError => "invalid or unsupported complex expression" , ComplexErrorKind :: UnsupportedRadix => "unsupported radix for conversion" , } } }
};
}
