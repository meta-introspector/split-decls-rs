// Generated macro for app_has_version (function)
macro_rules! Depcrateapp_has_version {
() => {
// Module: crate
// Provides: {"app_has_version"}
// Dependencies: {}
fn app_has_version (cmd : & clap :: Command) -> bool { cmd . get_version () . or_else (| | cmd . get_long_version ()) . is_some () }
};
}
