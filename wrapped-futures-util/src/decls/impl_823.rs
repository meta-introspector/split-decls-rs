macro_rules! deps {
    () => {
        FuturesOrdered!();
    };
}

macro_rules! impl_823 {
    () => {
        deps!();
        impl < T : Future > Unpin for FuturesOrdered < T > { }
    };
}

impl_823!()