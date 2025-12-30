// Generated macro for rustc_flags (function)
macro_rules! Depcrate_outputrustc_flags {
() => {
// Module: crate::output
// Provides: {"rustc_flags"}
// Dependencies: {}
# [doc = " The `rustc-flags` instruction tells Cargo to pass the given space-separated"] # [doc = " flags to the compiler."] # [doc = ""] # [doc = " This only allows the `-l` and `-L` flags, and is"] # [doc = " equivalent to using [`rustc_link_lib`] and [`rustc_link_search`]."] # [track_caller] pub fn rustc_flags (flags : & str) { if flags . contains ('\n') { panic ! ("cannot emit rustc-flags: invalid flags") ; } emit ("rustc-flags" , flags) ; }
};
}
