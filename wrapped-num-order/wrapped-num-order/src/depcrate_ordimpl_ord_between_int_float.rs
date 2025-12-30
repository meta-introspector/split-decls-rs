// Generated macro for impl_ord_between_int_float (macro)
macro_rules! Depcrate_ordimpl_ord_between_int_float {
() => {
// Module: crate::ord
// Provides: {"impl_ord_between_int_float"}
// Dependencies: {}
macro_rules ! impl_ord_between_int_float { ($ ($ float : ty | $ int : ty ;) *) => ($ (impl NumOrd <$ float > for $ int { # [inline] fn num_partial_cmp (& self , other : &$ float) -> Option < Ordering > { if other . is_nan () { None } else if other < & (<$ int >:: MIN as $ float) { Some (Ordering :: Greater) } else if other >= & (<$ int >:: MAX as $ float) { Some (Ordering :: Less) } else if other . e () >= 0 { self . partial_cmp (& (* other as $ int)) } else { let trunc = * other as $ int ; (* self , trunc as $ float) . partial_cmp (& (trunc , * other)) } } } impl NumOrd <$ int > for $ float { # [inline] fn num_partial_cmp (& self , other : &$ int) -> Option < Ordering > { if self . is_nan () { None } else if self < & (<$ int >:: MIN as $ float) { Some (Ordering :: Less) } else if self >= & (<$ int >:: MAX as $ float) { Some (Ordering :: Greater) } else if self . e () >= 0 { (* self as $ int) . partial_cmp (other) } else { let trunc = * other as $ int ; (trunc , * self) . partial_cmp (& (* other , trunc as $ float)) } } }) *) ; }
};
}
