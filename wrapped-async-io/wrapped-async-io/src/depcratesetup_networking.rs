// Generated macro for setup_networking (function)
macro_rules! Depcratesetup_networking {
() => {
// Module: crate
// Provides: {"setup_networking"}
// Dependencies: {}
# [inline] fn setup_networking () { # [cfg (windows)] { static INIT : std :: sync :: Once = std :: sync :: Once :: new () ; INIT . call_once (| | { let _ = rustix :: net :: wsa_startup () ; }) ; } }
};
}
