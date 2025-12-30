// Generated macro for impl_132 (impl)
macro_rules! Depcrate_almost_complete_rangeimpl_132 {
() => {
// Module: crate::almost_complete_range
// Provides: {"impl_132"}
// Dependencies: {}
impl EarlyLintPass for AlmostCompleteRange { fn check_expr (& mut self , cx : & EarlyContext < '_ > , e : & Expr) { if let ExprKind :: Range (Some (start) , Some (end) , RangeLimits :: HalfOpen) = & e . kind && is_incomplete_range (start , end) && ! e . span . in_external_macro (cx . sess () . source_map ()) { span_lint_and_then (cx , ALMOST_COMPLETE_RANGE , e . span , "almost complete ascii range" , | diag | { let ctxt = e . span . ctxt () ; if let Some (start) = walk_span_to_context (start . span , ctxt) && let Some (end) = walk_span_to_context (end . span , ctxt) && self . msrv . meets (msrvs :: RANGE_INCLUSIVE) { diag . span_suggestion (trim_span (cx . sess () . source_map () , start . between (end)) , "use an inclusive range" , "..=" . to_owned () , Applicability :: MaybeIncorrect ,) ; } } ,) ; } } fn check_pat (& mut self , cx : & EarlyContext < '_ > , p : & Pat) { if let PatKind :: Range (Some (start) , Some (end) , kind) = & p . kind && matches ! (kind . node , RangeEnd :: Excluded) && is_incomplete_range (start , end) && ! p . span . in_external_macro (cx . sess () . source_map ()) { span_lint_and_then (cx , ALMOST_COMPLETE_RANGE , p . span , "almost complete ascii range" , | diag | { diag . span_suggestion (kind . span , "use an inclusive range" , if self . msrv . meets (msrvs :: RANGE_INCLUSIVE) { "..=" . to_owned () } else { "..." . to_owned () } , Applicability :: MaybeIncorrect ,) ; } ,) ; } } extract_msrv_attr ! () ; }
};
}
