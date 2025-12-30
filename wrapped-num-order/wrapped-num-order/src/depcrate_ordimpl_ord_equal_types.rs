// Generated macro for impl_ord_equal_types (macro)
macro_rules! Depcrate_ordimpl_ord_equal_types {
() => {
// Module: crate::ord
// Provides: {"impl_ord_equal_types"}
// Dependencies: {}
macro_rules ! impl_ord_equal_types { ($ ($ t : ty) *) => ($ (impl NumOrd <$ t > for $ t { # [inline] fn num_partial_cmp (& self , other : &$ t) -> Option < Ordering > { self . partial_cmp (& other) } }) *) ; }
};
}
