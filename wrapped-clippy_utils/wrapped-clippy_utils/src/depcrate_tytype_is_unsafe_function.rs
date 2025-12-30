// Generated macro for type_is_unsafe_function (function)
macro_rules! Depcrate_tytype_is_unsafe_function {
() => {
// Module: crate::ty
// Provides: {"type_is_unsafe_function"}
// Dependencies: {}
# [doc = " Returns `true` if the given type is an `unsafe` function."] pub fn type_is_unsafe_function < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { ty . is_fn () && ty . fn_sig (cx . tcx) . safety () . is_unsafe () }
};
}
