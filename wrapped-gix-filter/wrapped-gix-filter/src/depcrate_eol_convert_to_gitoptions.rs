// Generated macro for Options (struct)
macro_rules! Depcrate_eol_convert_to_gitOptions {
() => {
// Module: crate::eol::convert_to_git
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Additional context for use with [`convert_to_git`][super::convert_to_git()]."] # [derive (Default , Copy , Clone)] pub struct Options < 'a > { # [doc = " How to perform round-trip checks."] pub round_trip_check : Option < RoundTripCheck < 'a > > , # [doc = " Configuration related to EOL."] pub config : crate :: eol :: Configuration , }
};
}
