macro_rules! deps {
    () => {
        Lazy!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < F > Unpin for Lazy < F > { }
    };
}

impl_147!();