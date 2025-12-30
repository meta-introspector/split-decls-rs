// Generated macro for impl_68 (impl)
macro_rules! Depcrate_eqimpl_68 {
() => {
// Module: crate::eq
// Provides: {"impl_68"}
// Dependencies: {}
impl FloatMargin for F64Margin { type F = f64 ; type I = i64 ; # [inline] fn zero () -> F64Margin { F64Margin { epsilon : 0.0 , ulps : 0 , } } fn epsilon (self , epsilon : f64) -> Self { F64Margin { epsilon , .. self } } fn ulps (self , ulps : i64) -> Self { F64Margin { ulps , .. self } } }
};
}
