// Generated macro for app_has_subcommands (function)
macro_rules! Depcrateapp_has_subcommands {
() => {
// Module: crate
// Provides: {"app_has_subcommands"}
// Dependencies: {}
fn app_has_subcommands (cmd : & clap :: Command) -> bool { cmd . get_subcommands () . any (| i | ! i . is_hide_set ()) }
};
}
