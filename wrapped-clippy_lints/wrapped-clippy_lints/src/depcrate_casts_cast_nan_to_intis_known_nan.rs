// Generated macro for is_known_nan (function)
macro_rules! Depcrate_casts_cast_nan_to_intis_known_nan {
() => {
// Module: crate::casts::cast_nan_to_int
// Provides: {"is_known_nan"}
// Dependencies: {}
fn is_known_nan (cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { match ConstEvalCtxt :: new (cx) . eval (e) { Some (Constant :: F64 (n)) => n . is_nan () , Some (Constant :: F32 (n)) => n . is_nan () , _ => false , } }
};
}
