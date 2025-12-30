// Generated macro for impl_ord_by_swap (macro)
macro_rules! Depcrate_ordimpl_ord_by_swap {
() => {
// Module: crate::ord
// Provides: {"impl_ord_by_swap"}
// Dependencies: {}
# [allow (unused_macros)] macro_rules ! impl_ord_by_swap { ($ ($ t1 : ty | $ t2 : ty ;) *) => ($ (impl NumOrd <$ t2 > for $ t1 { # [inline] fn num_partial_cmp (& self , other : &$ t2) -> Option < Ordering > { other . num_partial_cmp (self) . map (Ordering :: reverse) } }) *) ; }
};
}
