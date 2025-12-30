// Generated macro for check (function)
macro_rules! Depcrate_methods_useless_nonzero_new_uncheckedcheck {
() => {
// Module: crate::methods::useless_nonzero_new_unchecked
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < '_ > , func : & Expr < 'tcx > , args : & [Expr < '_ >] , msrv : Msrv) { if let ExprKind :: Path (QPath :: TypeRelative (ty , segment)) = func . kind && segment . ident . name == sym :: new_unchecked && let [init_arg] = args && is_inside_always_const_context (cx . tcx , expr . hir_id) && cx . typeck_results () . node_type (ty . hir_id) . is_diag_item (cx , sym :: NonZero) && msrv . meets (cx , msrvs :: CONST_UNWRAP) { let mut app = Applicability :: MachineApplicable ; let ty_str = snippet_with_applicability (cx , ty . span , "_" , & mut app) ; let msg = format ! ("`{ty_str}::new()` and `Option::unwrap()` can be safely used in a `const` context") ; let sugg = format ! ("{ty_str}::new({}).unwrap()" , snippet_with_applicability (cx , init_arg . span , "_" , & mut app)) ; if let Node :: Block (Block { stmts : [] , span : block_span , rules : BlockCheckMode :: UnsafeBlock (UnsafeSource :: UserProvided) , .. }) = cx . tcx . parent_hir_node (expr . hir_id) { if ! block_span . from_expansion () { span_lint_and_sugg (cx , USELESS_NONZERO_NEW_UNCHECKED , * block_span , msg , "use instead" , sugg , app ,) ; } } else { span_lint_and_then (cx , USELESS_NONZERO_NEW_UNCHECKED , expr . span , msg , | diagnostic | { diagnostic . span_suggestion (expr . span , "use instead" , sugg , app) . note ("the fixed expression does not require an `unsafe` context") ; }) ; } } }
};
}
