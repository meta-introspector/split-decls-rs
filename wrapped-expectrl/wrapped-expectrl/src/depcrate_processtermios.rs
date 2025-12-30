// Generated macro for Termios (trait)
macro_rules! Depcrate_processTermios {
() => {
// Module: crate::process
// Provides: {"Termios"}
// Dependencies: {}
# [doc = " Terminal configuration trait, used for IO configuration."] pub trait Termios { # [doc = " Verifies whether a [`std::io::Write`] will be repeated in output stream and be read by [`std::io::Read`]."] fn is_echo (& self) -> Result < bool > ; # [doc = " Configure a echo logic."] fn set_echo (& mut self , on : bool) -> Result < bool > ; }
};
}
