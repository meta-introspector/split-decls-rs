// Generated macro for get_expr_snippet (function)
macro_rules! Depcrate_unit_types_unit_argget_expr_snippet {
() => {
// Module: crate::unit_types::unit_arg
// Provides: {"get_expr_snippet"}
// Dependencies: {}
fn get_expr_snippet < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> Option < Sugg < 'tcx > > { let mut app = Applicability :: MachineApplicable ; let snip = Sugg :: hir_with_context (cx , expr , SyntaxContext :: root () , ".." , & mut app) ; if app != Applicability :: MachineApplicable { return None ; } Some (snip) }
};
}
