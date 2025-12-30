// Generated macro for SaturatingMul (trait)
macro_rules! Depcrate_ops_saturatingSaturatingMul {
() => {
// Module: crate::ops::saturating
// Provides: {"SaturatingMul"}
// Dependencies: {}
# [doc = " Performs multiplication that saturates at the numeric bounds instead of overflowing."] pub trait SaturatingMul : Sized + Mul < Self , Output = Self > { # [doc = " Saturating multiplication. Computes `self * other`, saturating at the relevant high or low boundary of"] # [doc = " the type."] fn saturating_mul (& self , v : & Self) -> Self ; }
};
}
