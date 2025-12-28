macro_rules! deps {
    () => {
        ControlFlow!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl ControlFlow for () { fn continuing () { } # [inline] fn should_break (& self) -> bool { false } # [inline] fn should_prune (& self) -> bool { false } }
    };
}

impl_34!();