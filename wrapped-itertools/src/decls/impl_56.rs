macro_rules! deps {
    () => {
        Itertools!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T > Itertools for T where T : Iterator + ? Sized { }
    };
}

impl_56!()