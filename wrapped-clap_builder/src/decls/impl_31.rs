macro_rules! deps {
    () => {
        Subcommand!();
        Command!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Subcommand for Infallible { fn augment_subcommands (cmd : Command) -> Command { cmd } fn augment_subcommands_for_update (cmd : Command) -> Command { cmd } fn has_subcommand (_name : & str) -> bool { false } }
    };
}

impl_31!()