// Generated macro for IntoTermion (trait)
macro_rules! DepcrateIntoTermion {
() => {
// Module: crate
// Provides: {"IntoTermion"}
// Dependencies: {}
# [doc = " A trait for converting a Ratatui type to a Termion type."] # [doc = ""] # [doc = " This trait is necessary to avoid the orphan rule, as we cannot implement a trait for a type"] # [doc = " defined in another crate."] pub trait IntoTermion < T > { # [doc = " Convert the Ratatui type to the Termion type."] fn into_termion (self) -> T ; }
};
}
