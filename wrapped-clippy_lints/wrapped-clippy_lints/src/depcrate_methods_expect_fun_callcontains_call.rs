// Generated macro for contains_call (function)
macro_rules! Depcrate_methods_expect_fun_callcontains_call {
() => {
// Module: crate::methods::expect_fun_call
// Provides: {"contains_call"}
// Dependencies: {}
fn contains_call < 'a > (cx : & LateContext < 'a > , arg : & 'a hir :: Expr < 'a >) -> bool { for_each_expr (cx , arg , | expr | { if matches ! (expr . kind , hir :: ExprKind :: MethodCall { .. } | hir :: ExprKind :: Call { .. }) && ! is_inside_always_const_context (cx . tcx , expr . hir_id) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
