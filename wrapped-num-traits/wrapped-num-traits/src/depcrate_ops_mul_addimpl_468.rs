// Generated macro for impl_468 (impl)
macro_rules! Depcrate_ops_mul_addimpl_468 {
() => {
// Module: crate::ops::mul_add
// Provides: {"impl_468"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "libm"))] impl MulAdd < f64 , f64 > for f64 { type Output = Self ; # [inline] fn mul_add (self , a : Self , b : Self) -> Self :: Output { < Self as crate :: Float > :: mul_add (self , a , b) } }
};
}
