// Generated macro for impl_4523 (impl)
macro_rules! Depcrate_map_unit_fnimpl_4523 {
() => {
// Module: crate::map_unit_fn
// Provides: {"impl_4523"}
// Dependencies: {}
impl LateLintPass < '_ > for MapUnit { fn check_stmt (& mut self , cx : & LateContext < '_ > , stmt : & hir :: Stmt < '_ >) { if let hir :: StmtKind :: Semi (expr) = stmt . kind && ! stmt . span . from_expansion () && let Some (arglists) = method_chain_args (expr , & [sym :: map]) { lint_map_unit_fn (cx , stmt , expr , arglists [0]) ; } } }
};
}
