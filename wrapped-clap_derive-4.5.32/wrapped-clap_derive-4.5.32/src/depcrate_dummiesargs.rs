// Generated macro for args (function)
macro_rules! Depcrate_dummiesargs {
() => {
// Module: crate::dummies
// Provides: {"args"}
// Dependencies: {}
# [must_use] pub (crate) fn args (name : & Ident) -> proc_macro2 :: TokenStream { let from_arg_matches = from_arg_matches (name) ; quote ! { # [automatically_derived] impl clap :: Args for # name { fn augment_args (_cmd : clap :: Command) -> clap :: Command { unimplemented ! () } fn augment_args_for_update (_cmd : clap :: Command) -> clap :: Command { unimplemented ! () } } # from_arg_matches } }
};
}
