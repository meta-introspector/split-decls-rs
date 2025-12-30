// Generated macro for impl_56 (impl)
macro_rules! Depcrate_eqimpl_56 {
() => {
// Module: crate::eq
// Provides: {"impl_56"}
// Dependencies: {}
impl FloatMargin for F32Margin { type F = f32 ; type I = i32 ; # [inline] fn zero () -> F32Margin { F32Margin { epsilon : 0.0 , ulps : 0 , } } fn epsilon (self , epsilon : f32) -> Self { F32Margin { epsilon , .. self } } fn ulps (self , ulps : i32) -> Self { F32Margin { ulps , .. self } } }
};
}
