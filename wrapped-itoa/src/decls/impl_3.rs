macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Copy for Buffer { }
    };
}

impl_3!()