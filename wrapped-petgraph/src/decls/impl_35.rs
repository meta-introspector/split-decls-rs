macro_rules! deps {
    () => {
        ControlFlow!();
        Control!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < B > ControlFlow for Control < B > { fn continuing () -> Self { Control :: Continue } fn should_break (& self) -> bool { matches ! (* self , Control :: Break (_)) } fn should_prune (& self) -> bool { matches ! (* self , Control :: Prune) } }
    };
}

impl_35!()