// Generated macro for IntoCrossterm (trait)
macro_rules! DepcrateIntoCrossterm {
() => {
// Module: crate
// Provides: {"IntoCrossterm"}
// Dependencies: {}
# [doc = " A trait for converting a Ratatui type to a Crossterm type."] # [doc = ""] # [doc = " This trait is needed for avoiding the orphan rule when implementing `From` for crossterm types"] # [doc = " once these are moved to a separate crate."] pub trait IntoCrossterm < C > { # [doc = " Converts the ratatui type to a crossterm type."] fn into_crossterm (self) -> C ; }
};
}
