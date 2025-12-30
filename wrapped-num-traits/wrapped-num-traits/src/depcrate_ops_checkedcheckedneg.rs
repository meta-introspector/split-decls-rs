// Generated macro for CheckedNeg (trait)
macro_rules! Depcrate_ops_checkedCheckedNeg {
() => {
// Module: crate::ops::checked
// Provides: {"CheckedNeg"}
// Dependencies: {}
# [doc = " Performs negation, returning `None` if the result can't be represented."] pub trait CheckedNeg : Sized { # [doc = " Negates a number, returning `None` for results that can't be represented, like signed `MIN`"] # [doc = " values that can't be positive, or non-zero unsigned values that can't be negative."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use num_traits::CheckedNeg;"] # [doc = " use std::i32::MIN;"] # [doc = ""] # [doc = " assert_eq!(CheckedNeg::checked_neg(&1_i32), Some(-1));"] # [doc = " assert_eq!(CheckedNeg::checked_neg(&-1_i32), Some(1));"] # [doc = " assert_eq!(CheckedNeg::checked_neg(&MIN), None);"] # [doc = ""] # [doc = " assert_eq!(CheckedNeg::checked_neg(&0_u32), Some(0));"] # [doc = " assert_eq!(CheckedNeg::checked_neg(&1_u32), None);"] # [doc = " ```"] fn checked_neg (& self) -> Option < Self > ; }
};
}
