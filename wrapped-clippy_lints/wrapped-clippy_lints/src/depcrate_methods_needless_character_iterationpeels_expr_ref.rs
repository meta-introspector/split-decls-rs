// Generated macro for peels_expr_ref (function)
macro_rules! Depcrate_methods_needless_character_iterationpeels_expr_ref {
() => {
// Module: crate::methods::needless_character_iteration
// Provides: {"peels_expr_ref"}
// Dependencies: {}
fn peels_expr_ref < 'a , 'tcx > (mut expr : & 'a Expr < 'tcx >) -> & 'a Expr < 'tcx > { while let ExprKind :: AddrOf (_ , _ , e) = expr . kind { expr = e ; } expr }
};
}
