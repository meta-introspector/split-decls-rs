// Generated macro for to_display_output (function)
macro_rules! Depcrate_matchers_eq_matcherto_display_output {
() => {
// Module: crate::matchers::eq_matcher
// Provides: {"to_display_output"}
// Dependencies: {}
fn to_display_output (string : & str) -> Option < String > { Some (string . strip_prefix ('"') ? . strip_suffix ('"') ? . split ("\\n") . collect :: < Vec < _ > > () . join ("\n")) }
};
}
