macro_rules! deps {
    () => {
        Bucket!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        unsafe impl < T > Send for Bucket < T > { }
    };
}

impl_46!()