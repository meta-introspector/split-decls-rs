// Generated macro for impl_94 (impl)
macro_rules! Depcrate_utils_format_args_collectorimpl_94 {
() => {
// Module: crate::utils::format_args_collector
// Provides: {"impl_94"}
// Dependencies: {}
impl EarlyLintPass for FormatArgsCollector { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: FormatArgs (args) = & expr . kind { if has_span_from_proc_macro (cx , args) { return ; } self . format_args . insert (expr . span . with_parent (None) , (* * args) . clone ()) ; } } fn check_crate_post (& mut self , _ : & EarlyContext < '_ > , _ : & Crate) { self . storage . set (mem :: take (& mut self . format_args)) ; } }
};
}
