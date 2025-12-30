// Generated macro for check_clone_on_copy (function)
macro_rules! Depcrate_non_canonical_implscheck_clone_on_copy {
() => {
// Module: crate::non_canonical_impls
// Provides: {"check_clone_on_copy"}
// Dependencies: {}
fn check_clone_on_copy (cx : & LateContext < '_ > , impl_item : & ImplItem < '_ > , block : & Block < '_ >) { if impl_item . ident . name == sym :: clone { if block . stmts . is_empty () && let Some (expr) = block . expr && let ExprKind :: Unary (UnOp :: Deref , deref) = expr . kind && let ExprKind :: Path (qpath) = deref . kind && last_path_segment (& qpath) . ident . name == kw :: SelfLower { return ; } if is_from_proc_macro (cx , impl_item) { return ; } span_lint_and_sugg (cx , NON_CANONICAL_CLONE_IMPL , block . span , "non-canonical implementation of `clone` on a `Copy` type" , "change this to" , "{ *self }" . to_owned () , Applicability :: MaybeIncorrect ,) ; } if impl_item . ident . name == sym :: clone_from && ! is_from_proc_macro (cx , impl_item) { span_lint_and_sugg (cx , NON_CANONICAL_CLONE_IMPL , impl_item . span , "unnecessary implementation of `clone_from` on a `Copy` type" , "remove it" , String :: new () , Applicability :: MaybeIncorrect ,) ; } }
};
}
