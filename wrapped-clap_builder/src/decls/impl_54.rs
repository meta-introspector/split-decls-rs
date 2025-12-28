macro_rules! deps {
    () => {
        Arg!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Eq for Arg { }
    };
}

impl_54!();