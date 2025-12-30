// Generated macro for is_any_expr_in_map_used (function)
macro_rules! Depcrate_entryis_any_expr_in_map_used {
() => {
// Module: crate::entry
// Provides: {"is_any_expr_in_map_used"}
// Dependencies: {}
# [doc = " Check if the given expression is used for each sub-expression in the given map."] # [doc = " For example, in map `a.b.c.my_map`, The expression `a.b.c.my_map`, `a.b.c`, `a.b`, and `a` are"] # [doc = " all checked."] fn is_any_expr_in_map_used < 'tcx > (cx : & LateContext < 'tcx > , spanless_eq : & mut SpanlessEq < '_ , 'tcx > , map : & 'tcx Expr < 'tcx > , expr : & 'tcx Expr < 'tcx > ,) -> bool { for_each_expr (cx , map , | e | { if spanless_eq . eq_expr (e , expr) { return ControlFlow :: Break (()) ; } ControlFlow :: Continue (()) }) . is_some () }
};
}
