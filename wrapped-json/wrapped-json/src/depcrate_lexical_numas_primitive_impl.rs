// Generated macro for as_primitive_impl (macro)
macro_rules! Depcrate_lexical_numas_primitive_impl {
() => {
// Module: crate::lexical::num
// Provides: {"as_primitive_impl"}
// Dependencies: {}
macro_rules ! as_primitive_impl { ($ ($ ty : ident) *) => { $ (impl AsPrimitive for $ ty { # [inline] fn as_u32 (self) -> u32 { self as u32 } # [inline] fn as_u64 (self) -> u64 { self as u64 } # [inline] fn as_u128 (self) -> u128 { self as u128 } # [inline] fn as_usize (self) -> usize { self as usize } # [inline] fn as_f32 (self) -> f32 { self as f32 } # [inline] fn as_f64 (self) -> f64 { self as f64 } }) * } ; }
};
}
