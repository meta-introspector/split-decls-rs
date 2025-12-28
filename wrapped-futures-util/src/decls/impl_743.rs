macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_743 {
    () => {
        deps!();
        impl < I > Unpin for Iter < I > { }
    };
}

impl_743!()