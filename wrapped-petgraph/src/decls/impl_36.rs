macro_rules! deps {
    () => {
        EdgeType!();
        Undirected!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl EdgeType for Undirected { # [inline] fn is_directed () -> bool { false } }
    };
}

impl_36!()