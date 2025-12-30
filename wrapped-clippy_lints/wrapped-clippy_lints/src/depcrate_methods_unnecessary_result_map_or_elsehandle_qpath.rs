// Generated macro for handle_qpath (function)
macro_rules! Depcrate_methods_unnecessary_result_map_or_elsehandle_qpath {
() => {
// Module: crate::methods::unnecessary_result_map_or_else
// Provides: {"handle_qpath"}
// Dependencies: {}
fn handle_qpath (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , def_arg : & Expr < '_ > , expected_hir_id : HirId , qpath : QPath < '_ > ,) { if let QPath :: Resolved (_ , path) = qpath && let hir :: def :: Res :: Local (hir_id) = path . res && expected_hir_id == hir_id { emit_lint (cx , expr , recv , def_arg) ; } }
};
}
