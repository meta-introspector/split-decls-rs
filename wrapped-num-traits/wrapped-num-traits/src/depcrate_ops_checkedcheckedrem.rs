// Generated macro for CheckedRem (trait)
macro_rules! Depcrate_ops_checkedCheckedRem {
() => {
// Module: crate::ops::checked
// Provides: {"CheckedRem"}
// Dependencies: {}
# [doc = " Performs integral remainder, returning `None` on division by zero or if"] # [doc = " overflow occurred."] pub trait CheckedRem : Sized + Rem < Self , Output = Self > { # [doc = " Finds the remainder of dividing two numbers, checking for overflow and"] # [doc = " division by zero. If any of that happens, `None` is returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use num_traits::CheckedRem;"] # [doc = " use std::i32::MIN;"] # [doc = ""] # [doc = " assert_eq!(CheckedRem::checked_rem(&10, &7), Some(3));"] # [doc = " assert_eq!(CheckedRem::checked_rem(&10, &-7), Some(3));"] # [doc = " assert_eq!(CheckedRem::checked_rem(&-10, &7), Some(-3));"] # [doc = " assert_eq!(CheckedRem::checked_rem(&-10, &-7), Some(-3));"] # [doc = ""] # [doc = " assert_eq!(CheckedRem::checked_rem(&10, &0), None);"] # [doc = ""] # [doc = " assert_eq!(CheckedRem::checked_rem(&MIN, &1), Some(0));"] # [doc = " assert_eq!(CheckedRem::checked_rem(&MIN, &-1), None);"] # [doc = " ```"] fn checked_rem (& self , v : & Self) -> Option < Self > ; }
};
}
