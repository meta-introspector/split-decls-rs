macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! get_help_flag {
    () => {
        deps!();
        pub (crate) fn get_help_flag (cmd : & Command) -> Option < Cow < 'static , str > > { if ! cmd . is_disable_help_flag_set () { Some (Cow :: Borrowed ("--help")) } else if let Some (flag) = get_user_help_flag (cmd) { Some (Cow :: Owned (flag)) } else if cmd . has_subcommands () && ! cmd . is_disable_help_subcommand_set () { Some (Cow :: Borrowed ("help")) } else { None } }
    };
}

get_help_flag!();