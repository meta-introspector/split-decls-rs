macro_rules! deps {
    () => {
        ControlFlow!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < C : ControlFlow , E > ControlFlow for Result < C , E > { fn continuing () -> Self { Ok (C :: continuing ()) } fn should_break (& self) -> bool { if let Ok (ref c) = * self { c . should_break () } else { true } } fn should_prune (& self) -> bool { if let Ok (ref c) = * self { c . should_prune () } else { false } } }
    };
}

impl_36!();