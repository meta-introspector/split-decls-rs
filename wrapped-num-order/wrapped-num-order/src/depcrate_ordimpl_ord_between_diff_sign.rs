// Generated macro for impl_ord_between_diff_sign (macro)
macro_rules! Depcrate_ordimpl_ord_between_diff_sign {
() => {
// Module: crate::ord
// Provides: {"impl_ord_between_diff_sign"}
// Dependencies: {}
macro_rules ! impl_ord_between_diff_sign { ($ ($ signed : ty => $ unsigned : ty ;) *) => ($ (impl NumOrd <$ signed > for $ unsigned { # [inline] fn num_partial_cmp (& self , other : &$ signed) -> Option < Ordering > { if other < & 0 { Some (Ordering :: Greater) } else { self . partial_cmp (&<$ unsigned >:: try_from (* other) . unwrap ()) } } } impl NumOrd <$ unsigned > for $ signed { # [inline] fn num_partial_cmp (& self , other : &$ unsigned) -> Option < Ordering > { if self < & 0 { Some (Ordering :: Less) } else { <$ unsigned >:: try_from (* self) . unwrap () . partial_cmp (other) } } }) *) ; }
};
}
