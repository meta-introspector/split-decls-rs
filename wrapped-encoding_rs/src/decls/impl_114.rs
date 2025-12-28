macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl Eq for Encoding { }
    };
}

impl_114!()