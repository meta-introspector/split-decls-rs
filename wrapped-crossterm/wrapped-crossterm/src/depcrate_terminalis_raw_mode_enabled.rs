// Generated macro for is_raw_mode_enabled (function)
macro_rules! Depcrate_terminalis_raw_mode_enabled {
() => {
// Module: crate::terminal
// Provides: {"is_raw_mode_enabled"}
// Dependencies: {}
# [doc = " Tells whether the raw mode is enabled."] # [doc = ""] # [doc = " Please have a look at the [raw mode](./index.html#raw-mode) section."] pub fn is_raw_mode_enabled () -> io :: Result < bool > { # [cfg (unix)] { Ok (sys :: is_raw_mode_enabled ()) } # [cfg (windows)] { sys :: is_raw_mode_enabled () } }
};
}
