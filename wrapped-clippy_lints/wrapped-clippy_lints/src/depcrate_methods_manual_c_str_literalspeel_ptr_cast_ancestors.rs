// Generated macro for peel_ptr_cast_ancestors (function)
macro_rules! Depcrate_methods_manual_c_str_literalspeel_ptr_cast_ancestors {
() => {
// Module: crate::methods::manual_c_str_literals
// Provides: {"peel_ptr_cast_ancestors"}
// Dependencies: {}
# [doc = " Same as `peel_ptr_cast`, but the other way around, by walking up the ancestor cast expressions:"] # [doc = ""] # [doc = " `foo(x.cast() as *const _)`"] # [doc = "      ^ given this `x` expression, returns the `foo(...)` expression"] fn peel_ptr_cast_ancestors < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < 'tcx >) -> & 'tcx Expr < 'tcx > { let mut prev = e ; for (_ , node) in cx . tcx . hir_parent_iter (e . hir_id) { if let Node :: Expr (e) = node && get_cast_target (e) . is_some () { prev = e ; } else { break ; } } prev }
};
}
