macro_rules! deps {
    () => {
        IterDelimited!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < I : Iterator > IterDelimited for I { }
    };
}

impl_52!();