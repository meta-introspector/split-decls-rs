// Generated macro for CheckedShl (trait)
macro_rules! Depcrate_ops_checkedCheckedShl {
() => {
// Module: crate::ops::checked
// Provides: {"CheckedShl"}
// Dependencies: {}
# [doc = " Performs shift left, returning `None` on shifts larger than or equal to"] # [doc = " the type width."] pub trait CheckedShl : Sized + Shl < u32 , Output = Self > { # [doc = " Checked shift left. Computes `self << rhs`, returning `None`"] # [doc = " if `rhs` is larger than or equal to the number of bits in `self`."] # [doc = ""] # [doc = " ```"] # [doc = " use num_traits::CheckedShl;"] # [doc = ""] # [doc = " let x: u16 = 0x0001;"] # [doc = ""] # [doc = " assert_eq!(CheckedShl::checked_shl(&x, 0),  Some(0x0001));"] # [doc = " assert_eq!(CheckedShl::checked_shl(&x, 1),  Some(0x0002));"] # [doc = " assert_eq!(CheckedShl::checked_shl(&x, 15), Some(0x8000));"] # [doc = " assert_eq!(CheckedShl::checked_shl(&x, 16), None);"] # [doc = " ```"] fn checked_shl (& self , rhs : u32) -> Option < Self > ; }
};
}
