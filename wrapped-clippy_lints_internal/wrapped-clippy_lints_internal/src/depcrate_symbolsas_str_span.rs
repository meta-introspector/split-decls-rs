// Generated macro for as_str_span (function)
macro_rules! Depcrate_symbolsas_str_span {
() => {
// Module: crate::symbols
// Provides: {"as_str_span"}
// Dependencies: {}
# [doc = " ```ignore"] # [doc = " symbol.as_str()"] # [doc = " //     ^^^^^^^^"] # [doc = " ```"] fn as_str_span (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Option < Span > { if let ExprKind :: MethodCall (_ , recv , [] , _) = expr . kind && let Some (method_def_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && internal_paths :: SYMBOL_AS_STR . matches (cx , method_def_id) { Some (recv . span . shrink_to_hi () . to (expr . span . shrink_to_hi ())) } else { None } }
};
}
