// Generated macro for repeat_zero_or_one_literals (function)
macro_rules! Depcrate_literalsrepeat_zero_or_one_literals {
() => {
// Module: crate::literals
// Provides: {"repeat_zero_or_one_literals"}
// Dependencies: {}
fn repeat_zero_or_one_literals < F : FnMut (& Expr , & mut Literals) > (e : & Expr , lits : & mut Literals , mut f : F ,) { let (mut lits2 , mut lits3) = (lits . clone () , lits . to_empty ()) ; lits3 . set_limit_size (lits . limit_size () / 2) ; f (e , & mut lits3) ; if lits3 . is_empty () || ! lits2 . cross_product (& lits3) { lits . cut () ; return ; } lits2 . add (Lit :: empty ()) ; if ! lits . union (lits2) { lits . cut () ; } }
};
}
