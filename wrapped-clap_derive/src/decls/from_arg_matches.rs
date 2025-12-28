macro_rules! from_arg_matches {
    () => {
        # [must_use] pub (crate) fn from_arg_matches (name : & Ident) -> proc_macro2 :: TokenStream { quote ! { # [automatically_derived] impl clap :: FromArgMatches for # name { fn from_arg_matches (_m : & clap :: ArgMatches) -> :: std :: result :: Result < Self , clap :: Error > { unimplemented ! () } fn update_from_arg_matches (& mut self , matches : & clap :: ArgMatches) -> :: std :: result :: Result < () , clap :: Error > { unimplemented ! () } } } }
    };
}

from_arg_matches!();