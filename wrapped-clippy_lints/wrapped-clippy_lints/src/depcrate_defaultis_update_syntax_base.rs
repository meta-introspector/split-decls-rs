// Generated macro for is_update_syntax_base (function)
macro_rules! Depcrate_defaultis_update_syntax_base {
() => {
// Module: crate::default
// Provides: {"is_update_syntax_base"}
// Dependencies: {}
# [doc = " Returns whether `expr` is the update syntax base: `Foo { a: 1, .. base }`"] fn is_update_syntax_base < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> bool { if let Some (parent) = get_parent_expr (cx , expr) && let ExprKind :: Struct (_ , _ , StructTailExpr :: Base (base)) = parent . kind { base . hir_id == expr . hir_id } else { false } }
};
}
