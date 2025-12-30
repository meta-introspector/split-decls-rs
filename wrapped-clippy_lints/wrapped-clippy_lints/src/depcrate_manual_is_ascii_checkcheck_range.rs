// Generated macro for check_range (function)
macro_rules! Depcrate_manual_is_ascii_checkcheck_range {
() => {
// Module: crate::manual_is_ascii_check
// Provides: {"check_range"}
// Dependencies: {}
fn check_range (start : & PatExpr < '_ > , end : & PatExpr < '_ >) -> CharRange { if let PatExprKind :: Lit { lit : start_lit , negated : false , } = & start . kind && let PatExprKind :: Lit { lit : end_lit , negated : false , } = & end . kind { check_lit_range (start_lit , end_lit) } else { CharRange :: Otherwise } }
};
}
