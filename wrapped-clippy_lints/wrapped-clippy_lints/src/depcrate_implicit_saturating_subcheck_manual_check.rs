// Generated macro for check_manual_check (function)
macro_rules! Depcrate_implicit_saturating_subcheck_manual_check {
() => {
// Module: crate::implicit_saturating_sub
// Provides: {"check_manual_check"}
// Dependencies: {}
# [expect (clippy :: too_many_arguments)] fn check_manual_check < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx > , condition : & BinOp , left_hand : & Expr < 'tcx > , right_hand : & Expr < 'tcx > , if_block : & Expr < 'tcx > , else_block : & Expr < 'tcx > , msrv : Msrv ,) { let ty = cx . typeck_results () . expr_ty (left_hand) ; if ty . is_numeric () && ! ty . is_signed () { match condition . node { BinOpKind :: Gt | BinOpKind :: Ge => check_gt (cx , condition . span , expr . span , left_hand , right_hand , if_block , else_block , msrv , matches ! (clippy_utils :: get_parent_expr (cx , expr) , Some (Expr { kind : ExprKind :: If (..) , .. })) ,) , BinOpKind :: Lt | BinOpKind :: Le => check_gt (cx , condition . span , expr . span , right_hand , left_hand , if_block , else_block , msrv , matches ! (clippy_utils :: get_parent_expr (cx , expr) , Some (Expr { kind : ExprKind :: If (..) , .. })) ,) , _ => { } , } } }
};
}
