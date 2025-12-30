// Generated macro for repeat_one_or_more_literals (function)
macro_rules! Depcrate_literalsrepeat_one_or_more_literals {
() => {
// Module: crate::literals
// Provides: {"repeat_one_or_more_literals"}
// Dependencies: {}
fn repeat_one_or_more_literals < F : FnMut (& Expr , & mut Literals) > (e : & Expr , lits : & mut Literals , mut f : F ,) { f (e , lits) ; lits . cut () ; }
};
}
