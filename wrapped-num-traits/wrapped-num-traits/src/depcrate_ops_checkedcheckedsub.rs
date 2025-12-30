// Generated macro for CheckedSub (trait)
macro_rules! Depcrate_ops_checkedCheckedSub {
() => {
// Module: crate::ops::checked
// Provides: {"CheckedSub"}
// Dependencies: {}
# [doc = " Performs subtraction, returning `None` if overflow occurred."] pub trait CheckedSub : Sized + Sub < Self , Output = Self > { # [doc = " Subtracts two numbers, checking for overflow. If overflow happens,"] # [doc = " `None` is returned."] fn checked_sub (& self , v : & Self) -> Option < Self > ; }
};
}
