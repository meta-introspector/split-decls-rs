// Generated macro for expand1_string (function)
macro_rules! Depcrate_terminfo_colorexpand1_string {
() => {
// Module: crate::terminfo::color
// Provides: {"expand1_string"}
// Dependencies: {}
# [doc = " Shortcut function for the `foreground()` and `background()` functions."] fn expand1_string < 'a , T > (v : u8) -> String where T : Capability < 'a > + AsRef < [u8] > , { expand1 :: < 'a , T > (v) . unwrap_or_else (| | String :: new ()) }
};
}
