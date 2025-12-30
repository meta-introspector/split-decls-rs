// Generated macro for is_trait_method (function)
macro_rules! Depcrateis_trait_method {
() => {
// Module: crate
// Provides: {"is_trait_method"}
// Dependencies: {}
# [doc = " Checks if the method call given in `expr` belongs to the given trait."] pub fn is_trait_method (cx : & LateContext < '_ > , expr : & Expr < '_ > , diag_item : Symbol) -> bool { cx . typeck_results () . type_dependent_def_id (expr . hir_id) . is_some_and (| did | is_diag_trait_item (cx , did , diag_item)) }
};
}
