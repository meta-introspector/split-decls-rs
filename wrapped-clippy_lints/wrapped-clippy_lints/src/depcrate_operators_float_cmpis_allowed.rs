// Generated macro for is_allowed (function)
macro_rules! Depcrate_operators_float_cmpis_allowed {
() => {
// Module: crate::operators::float_cmp
// Provides: {"is_allowed"}
// Dependencies: {}
fn is_allowed (val : & Constant) -> bool { match val { & Constant :: F32 (f) => f == 0.0 || f . is_infinite () , & Constant :: F64 (f) => f == 0.0 || f . is_infinite () , Constant :: Vec (vec) => vec . iter () . all (| f | match f { Constant :: F32 (f) => * f == 0.0 || (* f) . is_infinite () , Constant :: F64 (f) => * f == 0.0 || (* f) . is_infinite () , _ => false , }) , _ => false , } }
};
}
