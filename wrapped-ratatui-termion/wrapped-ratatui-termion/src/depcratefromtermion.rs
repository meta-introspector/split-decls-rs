// Generated macro for FromTermion (trait)
macro_rules! DepcrateFromTermion {
() => {
// Module: crate
// Provides: {"FromTermion"}
// Dependencies: {}
# [doc = " A trait for converting a Termion type to a Ratatui type."] # [doc = ""] # [doc = " This trait is necessary to avoid the orphan rule, as we cannot implement a trait for a type"] # [doc = " defined in another crate."] pub trait FromTermion < T > { # [doc = " Convert the Termion type to the Ratatui type."] fn from_termion (termion : T) -> Self ; }
};
}
