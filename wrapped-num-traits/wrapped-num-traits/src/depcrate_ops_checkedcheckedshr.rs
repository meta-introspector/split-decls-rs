// Generated macro for CheckedShr (trait)
macro_rules! Depcrate_ops_checkedCheckedShr {
() => {
// Module: crate::ops::checked
// Provides: {"CheckedShr"}
// Dependencies: {}
# [doc = " Performs shift right, returning `None` on shifts larger than or equal to"] # [doc = " the type width."] pub trait CheckedShr : Sized + Shr < u32 , Output = Self > { # [doc = " Checked shift right. Computes `self >> rhs`, returning `None`"] # [doc = " if `rhs` is larger than or equal to the number of bits in `self`."] # [doc = ""] # [doc = " ```"] # [doc = " use num_traits::CheckedShr;"] # [doc = ""] # [doc = " let x: u16 = 0x8000;"] # [doc = ""] # [doc = " assert_eq!(CheckedShr::checked_shr(&x, 0),  Some(0x8000));"] # [doc = " assert_eq!(CheckedShr::checked_shr(&x, 1),  Some(0x4000));"] # [doc = " assert_eq!(CheckedShr::checked_shr(&x, 15), Some(0x0001));"] # [doc = " assert_eq!(CheckedShr::checked_shr(&x, 16), None);"] # [doc = " ```"] fn checked_shr (& self , rhs : u32) -> Option < Self > ; }
};
}
