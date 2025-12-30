// Generated macro for CheckedMul (trait)
macro_rules! Depcrate_ops_checkedCheckedMul {
() => {
// Module: crate::ops::checked
// Provides: {"CheckedMul"}
// Dependencies: {}
# [doc = " Performs multiplication, returning `None` if overflow occurred."] pub trait CheckedMul : Sized + Mul < Self , Output = Self > { # [doc = " Multiplies two numbers, checking for overflow. If overflow happens,"] # [doc = " `None` is returned."] fn checked_mul (& self , v : & Self) -> Option < Self > ; }
};
}
