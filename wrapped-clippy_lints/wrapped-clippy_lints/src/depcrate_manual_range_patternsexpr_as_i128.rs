// Generated macro for expr_as_i128 (function)
macro_rules! Depcrate_manual_range_patternsexpr_as_i128 {
() => {
// Module: crate::manual_range_patterns
// Provides: {"expr_as_i128"}
// Dependencies: {}
fn expr_as_i128 (expr : & PatExpr < '_ >) -> Option < i128 > { if let PatExprKind :: Lit { lit , negated } = expr . kind && let LitKind :: Int (num , _) = lit . node { let n = i128 :: try_from (num . get ()) . ok () ? ; Some (if negated { - n } else { n }) } else { None } }
};
}
