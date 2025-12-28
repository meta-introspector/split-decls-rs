macro_rules! deps {
    () => {
        Row!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Eq for Row < '_ > { }
    };
}

impl_64!()