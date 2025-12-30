// Generated macro for check (function)
macro_rules! Depcrate_methods_is_emptycheck {
() => {
// Module: crate::methods::is_empty
// Provides: {"check"}
// Dependencies: {}
# [doc = " Expression whose initialization depend on a constant conditioned by a `#[cfg(…)]` directive will"] # [doc = " not trigger the lint."] pub (super) fn check (cx : & LateContext < '_ > , expr : & '_ Expr < '_ > , receiver : & Expr < '_ >) { if expr . span . in_external_macro (cx . sess () . source_map ()) || ! receiver . span . eq_ctxt (expr . span) { return ; } if let Some (parent) = get_parent_expr (cx , expr) && let Some (parent) = get_parent_expr (cx , parent) && is_inside_always_const_context (cx . tcx , expr . hir_id) && let Some (macro_call) = root_macro_call (parent . span) && is_assert_macro (cx , macro_call . def_id) { return ; } let init_expr = expr_or_init (cx , receiver) ; if ! receiver . span . eq_ctxt (init_expr . span) { return ; } if let Some (init_is_empty) = ConstEvalCtxt :: new (cx) . eval_is_empty (init_expr) { span_lint (cx , CONST_IS_EMPTY , expr . span , format ! ("this expression always evaluates to {init_is_empty:?}") ,) ; } }
};
}
