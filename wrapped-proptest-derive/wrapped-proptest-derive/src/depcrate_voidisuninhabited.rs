// Generated macro for IsUninhabited (trait)
macro_rules! Depcrate_voidIsUninhabited {
() => {
// Module: crate::void
// Provides: {"IsUninhabited"}
// Dependencies: {}
# [doc = " A trait for types for which it is possible to check if the modelled"] # [doc = " object is uninhabited or not. A `false` answer means that we can not"] # [doc = " tell for sure that the thing is uninhabited, not that we are 100%"] # [doc = " certain that it is inhabited."] pub trait IsUninhabited { # [doc = " Returns true if the given type is known to be uninhabited."] # [doc = " There may be more scenarios under which the type is uninhabited."] # [doc = " Thus, this is not a complete and exhaustive check."] fn is_uninhabited (& self) -> bool ; }
};
}
