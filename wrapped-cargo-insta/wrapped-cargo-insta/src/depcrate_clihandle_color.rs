// Generated macro for handle_color (function)
macro_rules! Depcrate_clihandle_color {
() => {
// Module: crate::cli
// Provides: {"handle_color"}
// Dependencies: {}
fn handle_color (color : Option < ColorWhen >) { match color { Some (ColorWhen :: Always) => { set_colors_enabled (true) ; } Some (ColorWhen :: Never) => { set_colors_enabled (false) ; } Some (ColorWhen :: Auto) | None => { } } }
};
}
