// Generated macro for option_help (function)
macro_rules! Depcrate_renderoption_help {
() => {
// Module: crate::render
// Provides: {"option_help"}
// Dependencies: {}
fn option_help (opt : & Arg) -> Option < & clap :: builder :: StyledStr > { if ! opt . is_hide_long_help_set () { let long_help = opt . get_long_help () ; if long_help . is_some () { return long_help ; } } if ! opt . is_hide_short_help_set () { return opt . get_help () ; } None }
};
}
