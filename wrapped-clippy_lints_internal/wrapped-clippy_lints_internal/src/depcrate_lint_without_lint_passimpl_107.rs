// Generated macro for impl_107 (impl)
macro_rules! Depcrate_lint_without_lint_passimpl_107 {
() => {
// Module: crate::lint_without_lint_pass
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for LintCollector < '_ , 'tcx > { type NestedFilter = nested_filter :: All ; fn visit_path (& mut self , path : & Path < '_ > , _ : HirId) { if path . segments . len () == 1 { self . output . insert (path . segments [0] . ident . name) ; } } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
