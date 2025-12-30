// Generated macro for check_range_switch (function)
macro_rules! Depcrate_rangescheck_range_switch {
() => {
// Module: crate::ranges
// Provides: {"check_range_switch"}
// Dependencies: {}
# [doc = " Check for a `kind` of range in `expr`, check for `predicate` on the end,"] # [doc = " and emit the `lint` with `msg` and the `operator`."] fn check_range_switch < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , kind : RangeLimits , predicate : impl for < 'hir > FnOnce (& LateContext < '_ > , & Expr < 'hir >) -> Option < & 'hir Expr < 'hir > > , lint : & 'static Lint , msg : & 'static str , operator : & str ,) { if let Some (range) = higher :: Range :: hir (cx , expr) && let higher :: Range { start , end : Some (end) , limits , span , } = range && span . can_be_used_for_suggestions () && limits == kind && let Some (y) = predicate (cx , end) && can_switch_ranges (cx , expr , kind , cx . typeck_results () . expr_ty (y)) { span_lint_and_then (cx , lint , span , msg , | diag | { let mut app = Applicability :: MachineApplicable ; let start = start . map_or (String :: new () , | x | { Sugg :: hir_with_applicability (cx , x , "<x>" , & mut app) . maybe_paren () . to_string () }) ; let end = Sugg :: hir_with_applicability (cx , y , "<y>" , & mut app) . maybe_paren () ; match span . with_source_text (cx , | src | src . starts_with ('(') && src . ends_with (')')) { Some (true) => { diag . span_suggestion (span , "use" , format ! ("({start}{operator}{end})") , app) ; } , Some (false) => { diag . span_suggestion (span , "use" , format ! ("{start}{operator}{end}") , app) ; } , None => { } , } }) ; } }
};
}
