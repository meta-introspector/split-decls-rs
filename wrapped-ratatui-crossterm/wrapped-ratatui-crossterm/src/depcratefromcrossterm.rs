// Generated macro for FromCrossterm (trait)
macro_rules! DepcrateFromCrossterm {
() => {
// Module: crate
// Provides: {"FromCrossterm"}
// Dependencies: {}
# [doc = " A trait for converting a Crossterm type to a Ratatui type."] # [doc = ""] # [doc = " This trait is needed for avoiding the orphan rule when implementing `From` for crossterm types"] # [doc = " once these are moved to a separate crate."] pub trait FromCrossterm < C > { # [doc = " Converts the crossterm type to a ratatui type."] fn from_crossterm (value : C) -> Self ; }
};
}
