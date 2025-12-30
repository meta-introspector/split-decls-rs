// Generated macro for IntoTermwiz (trait)
macro_rules! DepcrateIntoTermwiz {
() => {
// Module: crate
// Provides: {"IntoTermwiz"}
// Dependencies: {}
# [doc = " A trait for converting types from Ratatui to Termwiz."] # [doc = ""] # [doc = " This trait replaces the `Into` trait for converting types from Ratatui to Termwiz. It is"] # [doc = " necessary because the `Into` trait is not implemented for types defined in external crates."] pub trait IntoTermwiz < T > { # [doc = " Converts the given Ratatui type to the Termwiz type."] fn into_termwiz (self) -> T ; }
};
}
