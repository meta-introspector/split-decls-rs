// Generated macro for peel_ptr_cast (function)
macro_rules! Depcrate_methods_manual_c_str_literalspeel_ptr_cast {
() => {
// Module: crate::methods::manual_c_str_literals
// Provides: {"peel_ptr_cast"}
// Dependencies: {}
# [doc = " `x.cast()` -> `x`"] # [doc = " `x as *const _` -> `x`"] # [doc = " `x` -> `x` (returns the same expression for non-cast exprs)"] fn peel_ptr_cast < 'tcx > (e : & 'tcx Expr < 'tcx >) -> & 'tcx Expr < 'tcx > { get_cast_target (e) . map_or (e , peel_ptr_cast) }
};
}
