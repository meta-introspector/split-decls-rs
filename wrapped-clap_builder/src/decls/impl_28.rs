macro_rules! deps {
    () => {
        Command!();
        Args!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Args for () { fn augment_args (cmd : Command) -> Command { cmd } fn augment_args_for_update (cmd : Command) -> Command { cmd } }
    };
}

impl_28!()