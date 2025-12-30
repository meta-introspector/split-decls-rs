// Generated macro for impl_4392 (impl)
macro_rules! Depcrate_manual_rotateimpl_4392 {
() => {
// Module: crate::manual_rotate
// Provides: {"impl_4392"}
// Dependencies: {}
impl LateLintPass < '_ > for ManualRotate { fn check_expr < 'tcx > (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if let ExprKind :: Binary (op , l , r) = expr . kind && let BinOpKind :: Add | BinOpKind :: BitOr = op . node && let Some ((l_shift_dir , l_amount , l_expr)) = parse_shift (cx , l) && let Some ((r_shift_dir , r_amount , r_expr)) = parse_shift (cx , r) { if l_shift_dir == r_shift_dir { return ; } if ! clippy_utils :: eq_expr_value (cx , l_expr , r_expr) { return ; } let Some (bit_width) = (match cx . typeck_results () . expr_ty (expr) . kind () { ty :: Int (itype) => itype . bit_width () , ty :: Uint (itype) => itype . bit_width () , _ => return , }) else { return ; } ; if l_amount + r_amount == u128 :: from (bit_width) { let (shift_function , amount) = if l_amount < r_amount { (l_shift_dir , l_amount) } else { (r_shift_dir , r_amount) } ; let mut applicability = Applicability :: MachineApplicable ; let expr_sugg = sugg :: Sugg :: hir_with_applicability (cx , l_expr , "_" , & mut applicability) . maybe_paren () ; span_lint_and_sugg (cx , MANUAL_ROTATE , expr . span , "there is no need to manually implement bit rotation" , "this expression can be rewritten as" , format ! ("{expr_sugg}.{shift_function}({amount})") , Applicability :: MachineApplicable ,) ; } } } }
};
}
