// Generated macro for IntoRatatui (trait)
macro_rules! DepcrateIntoRatatui {
() => {
// Module: crate
// Provides: {"IntoRatatui"}
// Dependencies: {}
# [doc = " A replacement for the `Into` trait for converting types from Ratatui to Termwiz."] # [doc = ""] # [doc = " This trait is necessary because the `Into` trait is not implemented for types defined in"] # [doc = " external crates."] # [doc = ""] # [doc = " A blanket implementation is provided for all types that implement `FromTermwiz`."] # [doc = ""] # [doc = " This trait is private to the module as it would otherwise conflict with the other backend"] # [doc = " modules. It is mainly used to avoid rewriting all the `.into()` calls in this module."] trait IntoRatatui < R > { fn into_ratatui (self) -> R ; }
};
}
