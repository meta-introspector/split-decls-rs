// Generated macro for differ_by_one (function)
macro_rules! Depcrate_operators_manual_div_ceildiffer_by_one {
() => {
// Module: crate::operators::manual_div_ceil
// Provides: {"differ_by_one"}
// Dependencies: {}
# [doc = " Checks if two expressions represent non-zero integer literals such that `small_expr + 1 =="] # [doc = " large_expr`."] fn differ_by_one (small_expr : & Expr < '_ > , large_expr : & Expr < '_ >) -> bool { if let ExprKind :: Lit (small) = small_expr . kind && let ExprKind :: Lit (large) = large_expr . kind && let LitKind :: Int (s , _) = small . node && let LitKind :: Int (l , _) = large . node { Some (l . get ()) == s . get () . checked_add (1) } else if let ExprKind :: Unary (UnOp :: Neg , small_inner_expr) = small_expr . kind && let ExprKind :: Unary (UnOp :: Neg , large_inner_expr) = large_expr . kind { differ_by_one (large_inner_expr , small_inner_expr) } else { false } }
};
}
