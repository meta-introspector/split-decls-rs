// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
# [cfg (windows)] impl < Stream : AsHandle > IsTerminal for Stream { # [inline] fn is_terminal (& self) -> bool { handle_is_console (self . as_handle ()) } }
};
}
