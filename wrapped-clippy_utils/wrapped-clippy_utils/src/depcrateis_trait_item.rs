// Generated macro for is_trait_item (function)
macro_rules! Depcrateis_trait_item {
() => {
// Module: crate
// Provides: {"is_trait_item"}
// Dependencies: {}
# [doc = " Checks if the given expression is a path referring an item on the trait"] # [doc = " that is marked with the given diagnostic item."] # [doc = ""] # [doc = " For checking method call expressions instead of path expressions, use"] # [doc = " [`is_trait_method`]."] # [doc = ""] # [doc = " For example, this can be used to find if an expression like `u64::default`"] # [doc = " refers to an item of the trait `Default`, which is associated with the"] # [doc = " `diag_item` of `sym::Default`."] pub fn is_trait_item (cx : & LateContext < '_ > , expr : & Expr < '_ > , diag_item : Symbol) -> bool { if let ExprKind :: Path (ref qpath) = expr . kind { cx . qpath_res (qpath , expr . hir_id) . opt_def_id () . is_some_and (| def_id | is_diag_trait_item (cx , def_id , diag_item)) } else { false } }
};
}
