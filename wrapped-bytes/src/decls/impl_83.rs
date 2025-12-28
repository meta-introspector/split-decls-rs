macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl Eq for Bytes { }
    };
}

impl_83!()