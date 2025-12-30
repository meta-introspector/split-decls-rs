// Generated macro for FromTermwiz (trait)
macro_rules! DepcrateFromTermwiz {
() => {
// Module: crate
// Provides: {"FromTermwiz"}
// Dependencies: {}
# [doc = " A trait for converting types from Termwiz to Ratatui."] # [doc = ""] # [doc = " This trait replaces the `From` trait for converting types from Termwiz to Ratatui. It is"] # [doc = " necessary because the `From` trait is not implemented for types defined in external crates."] pub trait FromTermwiz < T > { # [doc = " Converts the given Termwiz type to the Ratatui type."] fn from_termwiz (termwiz : T) -> Self ; }
};
}
