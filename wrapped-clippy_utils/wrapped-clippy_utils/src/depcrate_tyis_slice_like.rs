// Generated macro for is_slice_like (function)
macro_rules! Depcrate_tyis_slice_like {
() => {
// Module: crate::ty
// Provides: {"is_slice_like"}
// Dependencies: {}
# [doc = " Check if `ty` is slice-like, i.e., `&[T]`, `[T; N]`, or `Vec<T>`."] pub fn is_slice_like < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { ty . is_slice () || ty . is_array () || ty . is_diag_item (cx , sym :: Vec) }
};
}
