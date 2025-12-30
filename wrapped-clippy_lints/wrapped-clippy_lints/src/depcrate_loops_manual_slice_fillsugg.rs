// Generated macro for sugg (function)
macro_rules! Depcrate_loops_manual_slice_fillsugg {
() => {
// Module: crate::loops::manual_slice_fill
// Provides: {"sugg"}
// Dependencies: {}
fn sugg < 'tcx > (cx : & LateContext < 'tcx > , body : & 'tcx Expr < '_ > , expr : & 'tcx Expr < '_ > , slice_span : rustc_span :: Span , assignval_span : rustc_span :: Span ,) { let mut app = if span_contains_comment (cx . sess () . source_map () , body . span) { Applicability :: MaybeIncorrect } else { Applicability :: MachineApplicable } ; span_lint_and_sugg (cx , MANUAL_SLICE_FILL , expr . span , "manually filling a slice" , "try" , format ! ("{}.fill({});" , snippet_with_applicability (cx , slice_span , ".." , & mut app) , snippet_with_applicability (cx , assignval_span , ".." , & mut app) ,) , app ,) ; }
};
}
