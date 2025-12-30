// Generated macro for SaturatingAdd (trait)
macro_rules! Depcrate_ops_saturatingSaturatingAdd {
() => {
// Module: crate::ops::saturating
// Provides: {"SaturatingAdd"}
// Dependencies: {}
# [doc = " Performs addition that saturates at the numeric bounds instead of overflowing."] pub trait SaturatingAdd : Sized + Add < Self , Output = Self > { # [doc = " Saturating addition. Computes `self + other`, saturating at the relevant high or low boundary of"] # [doc = " the type."] fn saturating_add (& self , v : & Self) -> Self ; }
};
}
