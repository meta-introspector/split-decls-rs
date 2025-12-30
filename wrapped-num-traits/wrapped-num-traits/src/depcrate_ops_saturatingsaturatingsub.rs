// Generated macro for SaturatingSub (trait)
macro_rules! Depcrate_ops_saturatingSaturatingSub {
() => {
// Module: crate::ops::saturating
// Provides: {"SaturatingSub"}
// Dependencies: {}
# [doc = " Performs subtraction that saturates at the numeric bounds instead of overflowing."] pub trait SaturatingSub : Sized + Sub < Self , Output = Self > { # [doc = " Saturating subtraction. Computes `self - other`, saturating at the relevant high or low boundary of"] # [doc = " the type."] fn saturating_sub (& self , v : & Self) -> Self ; }
};
}
