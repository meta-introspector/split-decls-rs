// Generated macro for CheckedDiv (trait)
macro_rules! Depcrate_ops_checkedCheckedDiv {
() => {
// Module: crate::ops::checked
// Provides: {"CheckedDiv"}
// Dependencies: {}
# [doc = " Performs division, returning `None` on division by zero or if overflow"] # [doc = " occurred."] pub trait CheckedDiv : Sized + Div < Self , Output = Self > { # [doc = " Divides two numbers, checking for overflow and division by"] # [doc = " zero. If any of that happens, `None` is returned."] fn checked_div (& self , v : & Self) -> Option < Self > ; }
};
}
