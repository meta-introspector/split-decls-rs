// Generated macro for is_neg_infinity (function)
macro_rules! Depcrate_manual_float_methodsis_neg_infinity {
() => {
// Module: crate::manual_float_methods
// Provides: {"is_neg_infinity"}
// Dependencies: {}
fn is_neg_infinity (constant : & Constant < '_ >) -> bool { match constant { Constant :: F32 (float) => * float == f32 :: NEG_INFINITY , Constant :: F64 (float) => * float == f64 :: NEG_INFINITY , _ => false , } }
};
}
