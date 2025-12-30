// Generated macro for NonBlocking (trait)
macro_rules! Depcrate_processNonBlocking {
() => {
// Module: crate::process
// Provides: {"NonBlocking"}
// Dependencies: {}
# [doc = " NonBlocking interface represens a [std::io::Read]er which can be turned in a non blocking mode"] # [doc = " so its read operations will return imideately."] pub trait NonBlocking { # [doc = " Sets a [std::io::Read]er into a non/blocking mode."] fn set_blocking (& mut self , on : bool) -> Result < () > ; }
};
}
