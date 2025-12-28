macro_rules! deps {
    () => {
        Queue!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Queue < T > { }
    };
}

impl_15!()