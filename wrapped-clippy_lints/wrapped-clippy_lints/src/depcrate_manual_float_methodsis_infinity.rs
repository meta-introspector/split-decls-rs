// Generated macro for is_infinity (function)
macro_rules! Depcrate_manual_float_methodsis_infinity {
() => {
// Module: crate::manual_float_methods
// Provides: {"is_infinity"}
// Dependencies: {}
fn is_infinity (constant : & Constant < '_ >) -> bool { match constant { Constant :: F32 (float) => * float == f32 :: INFINITY , Constant :: F64 (float) => * float == f64 :: INFINITY , _ => false , } }
};
}
