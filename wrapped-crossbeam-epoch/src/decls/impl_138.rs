macro_rules! deps {
    () => {
        Queue!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for Queue < T > { }
    };
}

impl_138!()