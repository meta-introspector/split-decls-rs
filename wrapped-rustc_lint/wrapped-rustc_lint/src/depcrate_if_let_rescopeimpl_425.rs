// Generated macro for impl_425 (impl)
macro_rules! Depcrate_if_let_rescopeimpl_425 {
() => {
// Module: crate::if_let_rescope
// Provides: {"impl_425"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for IfLetRescope { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx >) { if expr . span . edition () . at_least_rust_2024 () || cx . tcx . lints_that_dont_need_to_run (()) . contains (& LintId :: of (IF_LET_RESCOPE)) { return ; } if let hir :: ExprKind :: Loop (block , _label , hir :: LoopSource :: While , _span) = expr . kind && let Some (value) = block . expr && let hir :: ExprKind :: If (cond , _conseq , _alt) = value . kind && let hir :: ExprKind :: Let (..) = cond . kind { self . skip . insert (value . hir_id) ; return ; } if expr_parent_is_stmt (cx . tcx , expr . hir_id) && matches ! (expr . kind , hir :: ExprKind :: If (_cond , _conseq , None)) { return ; } self . probe_if_cascade (cx , expr) ; } }
};
}
