// Generated macro for check_inverted_bool_in_condition (function)
macro_rules! Depcrate_booleanscheck_inverted_bool_in_condition {
() => {
// Module: crate::booleans
// Provides: {"check_inverted_bool_in_condition"}
// Dependencies: {}
fn check_inverted_bool_in_condition (cx : & LateContext < '_ > , expr_span : Span , op : BinOpKind , left : & Expr < '_ > , right : & Expr < '_ > ,) { if expr_span . from_expansion () || ! cx . typeck_results () . node_types () [left . hir_id] . is_bool () || ! cx . typeck_results () . node_types () [right . hir_id] . is_bool () { return ; } let suggestion = match (left . kind , right . kind) { (ExprKind :: Unary (UnOp :: Not , left_sub) , ExprKind :: Unary (UnOp :: Not , right_sub)) => { let Some (left) = left_sub . span . get_source_text (cx) else { return ; } ; let Some (right) = right_sub . span . get_source_text (cx) else { return ; } ; let Some (op) = bin_op_eq_str (op) else { return } ; format ! ("{left} {op} {right}") } , (ExprKind :: Unary (UnOp :: Not , left_sub) , _) => { let Some (left) = left_sub . span . get_source_text (cx) else { return ; } ; let Some (right) = right . span . get_source_text (cx) else { return ; } ; let Some (op) = inverted_bin_op_eq_str (op) else { return } ; format ! ("{left} {op} {right}") } , (_ , ExprKind :: Unary (UnOp :: Not , right_sub)) => { let Some (left) = left . span . get_source_text (cx) else { return ; } ; let Some (right) = right_sub . span . get_source_text (cx) else { return ; } ; let Some (op) = inverted_bin_op_eq_str (op) else { return } ; format ! ("{left} {op} {right}") } , _ => return , } ; span_lint_and_sugg (cx , NONMINIMAL_BOOL , expr_span , "this boolean expression can be simplified" , "try" , suggestion , Applicability :: MachineApplicable ,) ; }
};
}
