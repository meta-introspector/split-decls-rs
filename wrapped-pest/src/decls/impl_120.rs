macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl Eq for Position < '_ > { }
    };
}

impl_120!()