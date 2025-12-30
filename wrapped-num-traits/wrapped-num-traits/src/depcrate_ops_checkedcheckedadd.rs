// Generated macro for CheckedAdd (trait)
macro_rules! Depcrate_ops_checkedCheckedAdd {
() => {
// Module: crate::ops::checked
// Provides: {"CheckedAdd"}
// Dependencies: {}
# [doc = " Performs addition, returning `None` if overflow occurred."] pub trait CheckedAdd : Sized + Add < Self , Output = Self > { # [doc = " Adds two numbers, checking for overflow. If overflow happens, `None` is"] # [doc = " returned."] fn checked_add (& self , v : & Self) -> Option < Self > ; }
};
}
