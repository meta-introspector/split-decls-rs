// Generated macro for impl_472 (impl)
macro_rules! Depcrate_ops_mul_addimpl_472 {
() => {
// Module: crate::ops::mul_add
// Provides: {"impl_472"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "libm"))] impl MulAddAssign < f32 , f32 > for f32 { # [inline] fn mul_add_assign (& mut self , a : Self , b : Self) { * self = < Self as crate :: Float > :: mul_add (* self , a , b) } }
};
}
