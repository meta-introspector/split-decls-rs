// Generated macro for contains_name (function)
macro_rules! Depcratecontains_name {
() => {
// Module: crate
// Provides: {"contains_name"}
// Dependencies: {}
# [doc = " Checks if an `Expr` contains a certain name."] pub fn contains_name < 'tcx > (name : Symbol , expr : & 'tcx Expr < '_ > , cx : & LateContext < 'tcx >) -> bool { let mut cn = ContainsName { cx , name } ; cn . visit_expr (expr) . is_break () }
};
}
