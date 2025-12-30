// Generated macro for impl_integer_type (macro)
macro_rules! Depcrate_operandsimpl_integer_type {
() => {
// Module: crate::operands
// Provides: {"impl_integer_type"}
// Dependencies: {}
macro_rules ! impl_integer_type { ($ ty : ident) => { impl From <$ ty > for PluralOperands { # [inline] fn from (input : $ ty) -> Self { Self { i : input as u64 , v : 0 , w : 0 , f : 0 , t : 0 , c : 0 , } } } } ; ($ ($ ty : ident) +) => { $ (impl_integer_type ! ($ ty) ;) + } ; }
};
}
