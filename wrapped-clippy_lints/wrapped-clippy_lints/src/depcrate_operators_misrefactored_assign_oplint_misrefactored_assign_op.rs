// Generated macro for lint_misrefactored_assign_op (function)
macro_rules! Depcrate_operators_misrefactored_assign_oplint_misrefactored_assign_op {
() => {
// Module: crate::operators::misrefactored_assign_op
// Provides: {"lint_misrefactored_assign_op"}
// Dependencies: {}
fn lint_misrefactored_assign_op (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , op : hir :: BinOpKind , rhs : & hir :: Expr < '_ > , assignee : & hir :: Expr < '_ > , rhs_other : & hir :: Expr < '_ > ,) { span_lint_and_then (cx , MISREFACTORED_ASSIGN_OP , expr . span , "variable appears on both sides of an assignment operation" , | diag | { if let Some (snip_a) = assignee . span . get_source_text (cx) && let Some (snip_r) = rhs_other . span . get_source_text (cx) { let a = & sugg :: Sugg :: hir (cx , assignee , "..") ; let r = & sugg :: Sugg :: hir (cx , rhs , "..") ; let long = format ! ("{snip_a} = {}" , sugg :: make_binop (op , a , r)) ; diag . span_suggestion (expr . span , format ! ("did you mean `{snip_a} = {snip_a} {} {snip_r}` or `{long}`? Consider replacing it with" , op . as_str ()) , format ! ("{snip_a} {}= {snip_r}" , op . as_str ()) , Applicability :: MaybeIncorrect ,) ; diag . span_suggestion (expr . span , "or" , long , Applicability :: MaybeIncorrect ,) ; } } ,) ; }
};
}
