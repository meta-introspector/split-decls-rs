// Generated macro for alternate_literals (function)
macro_rules! Depcrate_literalsalternate_literals {
() => {
// Module: crate::literals
// Provides: {"alternate_literals"}
// Dependencies: {}
fn alternate_literals < F : FnMut (& Expr , & mut Literals) > (es : & [Expr] , lits : & mut Literals , mut f : F ,) { let mut lits2 = lits . to_empty () ; for e in es { let mut lits3 = lits . to_empty () ; lits3 . set_limit_size (lits . limit_size () / 5) ; f (e , & mut lits3) ; if lits3 . is_empty () || ! lits2 . union (lits3) { lits . cut () ; return ; } } if ! lits . cross_product (& lits2) { lits . cut () ; } }
};
}
