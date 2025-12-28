macro_rules! deps {
    () => {
        Command!();
        Subcommand!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Subcommand for () { fn augment_subcommands (cmd : Command) -> Command { cmd } fn augment_subcommands_for_update (cmd : Command) -> Command { cmd } fn has_subcommand (_name : & str) -> bool { false } }
    };
}

impl_29!()