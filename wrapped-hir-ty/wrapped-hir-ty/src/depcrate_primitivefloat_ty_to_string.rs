// Generated macro for float_ty_to_string (function)
macro_rules! Depcrate_primitivefloat_ty_to_string {
() => {
// Module: crate::primitive
// Provides: {"float_ty_to_string"}
// Dependencies: {}
pub fn float_ty_to_string (ty : FloatTy) -> & 'static str { match ty { FloatTy :: F16 => "f16" , FloatTy :: F32 => "f32" , FloatTy :: F64 => "f64" , FloatTy :: F128 => "f128" , } }
};
}
