macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_872 {
    () => {
        deps!();
        impl < Fut > Unpin for FuturesUnordered < Fut > { }
    };
}

impl_872!();