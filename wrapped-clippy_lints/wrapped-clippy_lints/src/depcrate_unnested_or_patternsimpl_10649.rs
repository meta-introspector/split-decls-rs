// Generated macro for impl_10649 (impl)
macro_rules! Depcrate_unnested_or_patternsimpl_10649 {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"impl_10649"}
// Dependencies: {}
impl EarlyLintPass for UnnestedOrPatterns { fn check_arm (& mut self , cx : & EarlyContext < '_ > , a : & ast :: Arm) { if self . msrv . meets (msrvs :: OR_PATTERNS) { lint_unnested_or_patterns (cx , & a . pat) ; } } fn check_expr (& mut self , cx : & EarlyContext < '_ > , e : & ast :: Expr) { if self . msrv . meets (msrvs :: OR_PATTERNS) && let ast :: ExprKind :: Let (pat , _ , _ , _) = & e . kind { lint_unnested_or_patterns (cx , pat) ; } } fn check_param (& mut self , cx : & EarlyContext < '_ > , p : & ast :: Param) { if self . msrv . meets (msrvs :: OR_PATTERNS) { lint_unnested_or_patterns (cx , & p . pat) ; } } fn check_local (& mut self , cx : & EarlyContext < '_ > , l : & ast :: Local) { if self . msrv . meets (msrvs :: OR_PATTERNS) { lint_unnested_or_patterns (cx , & l . pat) ; } } extract_msrv_attr ! () ; }
};
}
