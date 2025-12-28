macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_774 {
    () => {
        deps!();
        impl < T > Unpin for Pending < T > { }
    };
}

impl_774!();