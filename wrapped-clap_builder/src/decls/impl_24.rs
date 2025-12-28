macro_rules! deps {
    () => {
        Command!();
        Args!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T : Args > Args for Box < T > { fn augment_args (cmd : Command) -> Command { < T as Args > :: augment_args (cmd) } fn augment_args_for_update (cmd : Command) -> Command { < T as Args > :: augment_args_for_update (cmd) } }
    };
}

impl_24!();