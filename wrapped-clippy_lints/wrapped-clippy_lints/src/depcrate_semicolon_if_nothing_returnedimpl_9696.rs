// Generated macro for impl_9696 (impl)
macro_rules! Depcrate_semicolon_if_nothing_returnedimpl_9696 {
() => {
// Module: crate::semicolon_if_nothing_returned
// Provides: {"impl_9696"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for SemicolonIfNothingReturned { fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & 'tcx Block < 'tcx >) { if ! block . span . from_expansion () && let Some (expr) = block . expr && ! from_attr_macro (expr . span) && let t_expr = cx . typeck_results () . expr_ty (expr) && t_expr . is_unit () && let mut app = Applicability :: MachineApplicable && let snippet = snippet_with_context (cx , expr . span , block . span . ctxt () , "}" , & mut app) . 0 && ! snippet . ends_with ('}') && ! snippet . ends_with (';') && cx . sess () . source_map () . is_multiline (block . span) { if let ExprKind :: DropTemps (..) = & expr . kind { return ; } span_lint_and_sugg (cx , SEMICOLON_IF_NOTHING_RETURNED , expr . span . source_callsite () , "consider adding a `;` to the last statement for consistent formatting" , "add a `;` here" , format ! ("{snippet};") , app ,) ; } } }
};
}
