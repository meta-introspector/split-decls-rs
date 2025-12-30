// Generated macro for Saturating (trait)
macro_rules! Depcrate_ops_saturatingSaturating {
() => {
// Module: crate::ops::saturating
// Provides: {"Saturating"}
// Dependencies: {}
# [doc = " Saturating math operations. Deprecated, use `SaturatingAdd`, `SaturatingSub` and"] # [doc = " `SaturatingMul` instead."] pub trait Saturating { # [doc = " Saturating addition operator."] # [doc = " Returns a+b, saturating at the numeric bounds instead of overflowing."] fn saturating_add (self , v : Self) -> Self ; # [doc = " Saturating subtraction operator."] # [doc = " Returns a-b, saturating at the numeric bounds instead of overflowing."] fn saturating_sub (self , v : Self) -> Self ; }
};
}
