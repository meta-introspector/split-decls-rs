macro_rules! deps {
    () => {
        ImportMap!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl Eq for ImportMap { }
    };
}

impl_423!();