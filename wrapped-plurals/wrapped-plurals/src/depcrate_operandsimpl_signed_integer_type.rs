// Generated macro for impl_signed_integer_type (macro)
macro_rules! Depcrate_operandsimpl_signed_integer_type {
() => {
// Module: crate::operands
// Provides: {"impl_signed_integer_type"}
// Dependencies: {}
macro_rules ! impl_signed_integer_type { ($ ty : ident) => { impl From <$ ty > for PluralOperands { # [inline] fn from (input : $ ty) -> Self { input . unsigned_abs () . into () } } } ; ($ ($ ty : ident) +) => { $ (impl_signed_integer_type ! ($ ty) ;) + } ; }
};
}
