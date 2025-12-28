macro_rules! deps {
    () => {
        Injector!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for Injector < T > { }
    };
}

impl_36!();