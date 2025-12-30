// Generated macro for impl_262 (impl)
macro_rules! Depcrate_expanded_nodes_expanded_stmtimpl_262 {
() => {
// Module: crate::expanded_nodes::expanded_stmt
// Provides: {"impl_262"}
// Dependencies: {}
impl CfgFalseExpandable for ExpandedStmt { fn expand_cfg_false < D : CfgFalseReporterContext > (& mut self , collector : & mut D , pos : usize , span : Span) { self . visit_stmt_attrs (| attrs | { attrs . remove (pos) ; }) ; } }
};
}
