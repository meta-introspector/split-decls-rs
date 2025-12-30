// Generated macro for impl_fixed_integer_from_integer_type (macro)
macro_rules! Depcrate_integerimpl_fixed_integer_from_integer_type {
() => {
// Module: crate::integer
// Provides: {"impl_fixed_integer_from_integer_type"}
// Dependencies: {}
macro_rules ! impl_fixed_integer_from_integer_type { ($ type : ident) => { impl From <$ type > for FixedInteger { fn from (value : $ type) -> Self { FixedInteger (Decimal :: from (value)) } } } ; }
};
}
