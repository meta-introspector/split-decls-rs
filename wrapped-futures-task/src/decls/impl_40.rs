macro_rules! deps {
    () => {
        FutureObj!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T > Unpin for FutureObj < '_ , T > { }
    };
}

impl_40!()