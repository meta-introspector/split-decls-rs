macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < T > Unpin for Pending < T > { }
    };
}

impl_156!()