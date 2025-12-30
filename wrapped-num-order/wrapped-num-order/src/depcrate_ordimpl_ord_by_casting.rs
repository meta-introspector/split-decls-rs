// Generated macro for impl_ord_by_casting (macro)
macro_rules! Depcrate_ordimpl_ord_by_casting {
() => {
// Module: crate::ord
// Provides: {"impl_ord_by_casting"}
// Dependencies: {}
macro_rules ! impl_ord_by_casting { ($ ($ small : ty => $ big : ty ;) *) => ($ (impl NumOrd <$ small > for $ big { # [inline] fn num_partial_cmp (& self , other : &$ small) -> Option < Ordering > { self . partial_cmp (&<$ big >:: from (* other)) } } impl NumOrd <$ big > for $ small { # [inline] fn num_partial_cmp (& self , other : &$ big) -> Option < Ordering > { <$ big >:: from (* self) . partial_cmp (other) } }) *) ; }
};
}
