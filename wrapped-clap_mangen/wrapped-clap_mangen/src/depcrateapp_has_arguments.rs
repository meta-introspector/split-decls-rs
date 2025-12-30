// Generated macro for app_has_arguments (function)
macro_rules! Depcrateapp_has_arguments {
() => {
// Module: crate
// Provides: {"app_has_arguments"}
// Dependencies: {}
fn app_has_arguments (cmd : & clap :: Command) -> bool { cmd . get_arguments () . any (| i | ! i . is_hide_set ()) }
};
}
