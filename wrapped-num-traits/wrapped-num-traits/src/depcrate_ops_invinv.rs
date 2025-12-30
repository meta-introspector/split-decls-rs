// Generated macro for Inv (trait)
macro_rules! Depcrate_ops_invInv {
() => {
// Module: crate::ops::inv
// Provides: {"Inv"}
// Dependencies: {}
# [doc = " Unary operator for retrieving the multiplicative inverse, or reciprocal, of a value."] pub trait Inv { # [doc = " The result after applying the operator."] type Output ; # [doc = " Returns the multiplicative inverse of `self`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::f64::INFINITY;"] # [doc = " use num_traits::Inv;"] # [doc = ""] # [doc = " assert_eq!(7.0.inv() * 7.0, 1.0);"] # [doc = " assert_eq!((-0.0).inv(), -INFINITY);"] # [doc = " ```"] fn inv (self) -> Self :: Output ; }
};
}
