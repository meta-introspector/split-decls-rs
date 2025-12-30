// Generated macro for expand1 (function)
macro_rules! Depcrate_terminfo_colorexpand1 {
() => {
// Module: crate::terminfo::color
// Provides: {"expand1"}
// Dependencies: {}
# [doc = " Shortcut function for the `foreground()` and `background()` functions."] fn expand1 < 'a , T > (v : u8) -> Option < String > where T : Capability < 'a > + AsRef < [u8] > , { let info = (* TERMINFO) . as_ref () ? ; let e = expand ! (info . get ::< T > () ?. as_ref () ; v) . ok () ? ; let s = std :: str :: from_utf8 (& e) . ok () ? ; Some (s . to_owned ()) }
};
}
