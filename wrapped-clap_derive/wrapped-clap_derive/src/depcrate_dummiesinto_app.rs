// Generated macro for into_app (function)
macro_rules! Depcrate_dummiesinto_app {
() => {
// Module: crate::dummies
// Provides: {"into_app"}
// Dependencies: {}
# [must_use] pub (crate) fn into_app (name : & Ident) -> proc_macro2 :: TokenStream { quote ! { # [automatically_derived] impl clap :: CommandFactory for # name { fn command <'b > () -> clap :: Command { unimplemented ! () } fn command_for_update <'b > () -> clap :: Command { unimplemented ! () } } } }
};
}
