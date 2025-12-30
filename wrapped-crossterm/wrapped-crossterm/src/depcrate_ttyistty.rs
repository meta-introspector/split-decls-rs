// Generated macro for IsTty (trait)
macro_rules! Depcrate_ttyIsTty {
() => {
// Module: crate::tty
// Provides: {"IsTty"}
// Dependencies: {}
# [doc = " Adds the `is_tty` method to types that might represent a terminal"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::io::stdout;"] # [doc = " use crossterm::tty::IsTty;"] # [doc = ""] # [doc = " let is_tty: bool = stdout().is_tty();"] # [doc = " ```"] pub trait IsTty { # [doc = " Returns true when an instance is a terminal teletype, otherwise false."] fn is_tty (& self) -> bool ; }
};
}
