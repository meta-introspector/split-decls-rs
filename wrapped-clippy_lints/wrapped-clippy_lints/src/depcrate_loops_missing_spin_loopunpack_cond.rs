// Generated macro for unpack_cond (function)
macro_rules! Depcrate_loops_missing_spin_loopunpack_cond {
() => {
// Module: crate::loops::missing_spin_loop
// Provides: {"unpack_cond"}
// Dependencies: {}
fn unpack_cond < 'tcx > (cond : & 'tcx Expr < 'tcx >) -> & 'tcx Expr < 'tcx > { match & cond . kind { ExprKind :: Block (Block { stmts : [] , expr : Some (e) , .. } , _ ,) | ExprKind :: Unary (_ , e) => unpack_cond (e) , ExprKind :: Binary (_ , l , r) => { let l = unpack_cond (l) ; if let ExprKind :: MethodCall (..) = l . kind { l } else { unpack_cond (r) } } , _ => cond , } }
};
}
