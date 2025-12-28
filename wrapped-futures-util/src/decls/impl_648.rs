macro_rules! deps {
    () => {
        Single!();
    };
}

macro_rules! impl_648 {
    () => {
        deps!();
        impl < T > Unpin for Single < T > { }
    };
}

impl_648!();