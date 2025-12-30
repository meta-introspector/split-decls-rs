// Generated macro for is_unsafe_fn (function)
macro_rules! Depcrate_tyis_unsafe_fn {
() => {
// Module: crate::ty
// Provides: {"is_unsafe_fn"}
// Dependencies: {}
# [doc = " Returns `true` if `ty` denotes an `unsafe fn`."] pub fn is_unsafe_fn < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { ty . is_fn () && ty . fn_sig (cx . tcx) . safety () . is_unsafe () }
};
}
