macro_rules! deps {
    () => {
        EdgeType!();
        Directed!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl EdgeType for Directed { # [inline] fn is_directed () -> bool { true } }
    };
}

impl_35!()