macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T > Unpin for Pending < T > { }
    };
}

impl_56!();