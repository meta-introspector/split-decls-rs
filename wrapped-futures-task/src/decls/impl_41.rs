macro_rules! deps {
    () => {
        FutureObj!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        unsafe impl < T > Send for FutureObj < '_ , T > { }
    };
}

impl_41!()