// Generated macro for is_expr_used_with_type_annotation (function)
macro_rules! Depcrate_operators_identity_opis_expr_used_with_type_annotation {
() => {
// Module: crate::operators::identity_op
// Provides: {"is_expr_used_with_type_annotation"}
// Dependencies: {}
fn is_expr_used_with_type_annotation < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> bool { match expr_use_ctxt (cx , expr) . use_node (cx) { ExprUseNode :: LetStmt (letstmt) => letstmt . ty . is_some () , ExprUseNode :: Return (_) => true , _ => false , } }
};
}
