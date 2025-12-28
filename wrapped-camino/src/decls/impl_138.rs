macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl Eq for Utf8Path { }
    };
}

impl_138!()