macro_rules! into_app {
    () => {
        # [must_use] pub (crate) fn into_app (name : & Ident) -> proc_macro2 :: TokenStream { quote ! { # [automatically_derived] impl clap :: CommandFactory for # name { fn command <'b > () -> clap :: Command { unimplemented ! () } fn command_for_update <'b > () -> clap :: Command { unimplemented ! () } } } }
    };
}

into_app!()