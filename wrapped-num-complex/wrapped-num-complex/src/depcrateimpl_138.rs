// Generated macro for impl_138 (impl)
macro_rules! Depcrateimpl_138 {
() => {
// Module: crate
// Provides: {"impl_138"}
// Dependencies: {}
impl < E > ParseComplexError < E > { fn expr_error () -> Self { ParseComplexError { kind : ComplexErrorKind :: ExprError , } } fn unsupported_radix () -> Self { ParseComplexError { kind : ComplexErrorKind :: UnsupportedRadix , } } fn from_error (error : E) -> Self { ParseComplexError { kind : ComplexErrorKind :: ParseError (error) , } } }
};
}
