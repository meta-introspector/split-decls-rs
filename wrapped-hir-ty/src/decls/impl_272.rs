macro_rules! deps {
    () => {
        Diverges!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl Diverges { fn is_always (self) -> bool { self == Diverges :: Always } }
    };
}

impl_272!()