// Generated macro for as_some_expr (function)
macro_rules! Depcrateas_some_expr {
() => {
// Module: crate
// Provides: {"as_some_expr"}
// Dependencies: {}
# [doc = " If `expr` is `Some(inner)`, returns `inner`"] pub fn as_some_expr < 'tcx > (cx : & LateContext < '_ > , expr : & 'tcx Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { if let ExprKind :: Call (e , [arg]) = expr . kind && e . res (cx) . ctor_parent (cx) . is_lang_item (cx , OptionSome) { Some (arg) } else { None } }
};
}
