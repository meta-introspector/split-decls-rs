macro_rules! deps {
    () => {
        CommandFactory!();
        Command!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T : CommandFactory > CommandFactory for Box < T > { fn command () -> Command { < T as CommandFactory > :: command () } fn command_for_update () -> Command { < T as CommandFactory > :: command_for_update () } }
    };
}

impl_22!()