macro_rules! deps {
    () => {
        Captures!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < T > Captures < '_ > for T { }
    };
}

impl_84!();