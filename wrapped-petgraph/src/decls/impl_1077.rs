macro_rules! deps {
    () => {
        Directed!();
        EdgeType!();
    };
}

macro_rules! impl_1077 {
    () => {
        deps!();
        impl EdgeType for Directed { # [inline] fn is_directed () -> bool { true } }
    };
}

impl_1077!();