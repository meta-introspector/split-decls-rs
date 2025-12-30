// Generated macro for check (function)
macro_rules! Depcrate_unit_types_unit_argcheck {
() => {
// Module: crate::unit_types::unit_arg
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if expr . span . from_expansion () { return ; } if is_questionmark_desugar_marked_call (expr) { return ; } if let Node :: Expr (parent_expr) = cx . tcx . parent_hir_node (expr . hir_id) && is_questionmark_desugar_marked_call (parent_expr) { return ; } let (receiver , args) = match expr . kind { ExprKind :: Call (_ , args) => (None , args) , ExprKind :: MethodCall (_ , receiver , args , _) => (Some (receiver) , args) , _ => return , } ; let args_to_recover = receiver . into_iter () . chain (args) . filter (| arg | { if cx . typeck_results () . expr_ty (arg) . is_unit () && ! utils :: is_unit_literal (arg) { ! matches ! (& arg . kind , ExprKind :: Match (.., MatchSource :: TryDesugar (_)) | ExprKind :: Path (..)) } else { false } }) . collect :: < Vec < _ > > () ; if ! args_to_recover . is_empty () && ! is_from_proc_macro (cx , expr) { lint_unit_args (cx , expr , args_to_recover . as_slice ()) ; } }
};
}
