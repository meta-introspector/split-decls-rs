// Generated macro for MulAddAssign (trait)
macro_rules! Depcrate_ops_mul_addMulAddAssign {
() => {
// Module: crate::ops::mul_add
// Provides: {"MulAddAssign"}
// Dependencies: {}
# [doc = " The fused multiply-add assignment operation `*self = (*self * a) + b`"] pub trait MulAddAssign < A = Self , B = Self > { # [doc = " Performs the fused multiply-add assignment operation `*self = (*self * a) + b`"] fn mul_add_assign (& mut self , a : A , b : B) ; }
};
}
