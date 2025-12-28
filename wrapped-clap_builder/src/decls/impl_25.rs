macro_rules! deps {
    () => {
        Subcommand!();
        Command!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T : Subcommand > Subcommand for Box < T > { fn augment_subcommands (cmd : Command) -> Command { < T as Subcommand > :: augment_subcommands (cmd) } fn augment_subcommands_for_update (cmd : Command) -> Command { < T as Subcommand > :: augment_subcommands_for_update (cmd) } fn has_subcommand (name : & str) -> bool { < T as Subcommand > :: has_subcommand (name) } }
    };
}

impl_25!()