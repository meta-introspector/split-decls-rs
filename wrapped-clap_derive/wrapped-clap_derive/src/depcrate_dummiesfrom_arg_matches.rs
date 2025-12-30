// Generated macro for from_arg_matches (function)
macro_rules! Depcrate_dummiesfrom_arg_matches {
() => {
// Module: crate::dummies
// Provides: {"from_arg_matches"}
// Dependencies: {}
# [must_use] pub (crate) fn from_arg_matches (name : & Ident) -> proc_macro2 :: TokenStream { quote ! { # [automatically_derived] impl clap :: FromArgMatches for # name { fn from_arg_matches (_m : & clap :: ArgMatches) -> :: std :: result :: Result < Self , clap :: Error > { unimplemented ! () } fn update_from_arg_matches (& mut self , matches : & clap :: ArgMatches) -> :: std :: result :: Result < () , clap :: Error > { unimplemented ! () } } } }
};
}
