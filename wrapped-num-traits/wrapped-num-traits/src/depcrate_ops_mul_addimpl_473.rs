// Generated macro for impl_473 (impl)
macro_rules! Depcrate_ops_mul_addimpl_473 {
() => {
// Module: crate::ops::mul_add
// Provides: {"impl_473"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "libm"))] impl MulAddAssign < f64 , f64 > for f64 { # [inline] fn mul_add_assign (& mut self , a : Self , b : Self) { * self = < Self as crate :: Float > :: mul_add (* self , a , b) } }
};
}
