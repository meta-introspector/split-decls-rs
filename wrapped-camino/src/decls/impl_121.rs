macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl Eq for Utf8Path { }
    };
}

impl_121!()