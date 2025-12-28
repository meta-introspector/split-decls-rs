macro_rules! deps {
    () => {
        DummyWaker!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl Wake for DummyWaker { fn wake (self : Arc < Self >) { } }
    };
}

impl_81!()