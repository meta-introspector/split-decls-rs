// Generated macro for impl_467 (impl)
macro_rules! Depcrate_ops_mul_addimpl_467 {
() => {
// Module: crate::ops::mul_add
// Provides: {"impl_467"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "libm"))] impl MulAdd < f32 , f32 > for f32 { type Output = Self ; # [inline] fn mul_add (self , a : Self , b : Self) -> Self :: Output { < Self as crate :: Float > :: mul_add (self , a , b) } }
};
}
