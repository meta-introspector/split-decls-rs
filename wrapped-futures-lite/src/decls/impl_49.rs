macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < I > Unpin for Iter < I > { }
    };
}

impl_49!()