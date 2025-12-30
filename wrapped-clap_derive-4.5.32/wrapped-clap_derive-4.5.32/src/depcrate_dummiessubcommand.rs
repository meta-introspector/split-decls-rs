// Generated macro for subcommand (function)
macro_rules! Depcrate_dummiessubcommand {
() => {
// Module: crate::dummies
// Provides: {"subcommand"}
// Dependencies: {}
# [must_use] pub (crate) fn subcommand (name : & Ident) -> proc_macro2 :: TokenStream { let from_arg_matches = from_arg_matches (name) ; quote ! { # [automatically_derived] impl clap :: Subcommand for # name { fn augment_subcommands (_cmd : clap :: Command) -> clap :: Command { unimplemented ! () } fn augment_subcommands_for_update (_cmd : clap :: Command) -> clap :: Command { unimplemented ! () } fn has_subcommand (name : & str) -> bool { unimplemented ! () } } # from_arg_matches } }
};
}
