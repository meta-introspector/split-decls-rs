// Generated macro for option_environment (function)
macro_rules! Depcrate_renderoption_environment {
() => {
// Module: crate::render
// Provides: {"option_environment"}
// Dependencies: {}
fn option_environment (opt : & Arg) -> Option < Vec < Inline > > { if opt . is_hide_env_set () { return None ; } else if let Some (env) = opt . get_env () { return Some (vec ! [roman ("May also be specified with the ") , bold (env . to_string_lossy () . into_owned ()) , roman (" environment variable. ") ,]) ; } None }
};
}
