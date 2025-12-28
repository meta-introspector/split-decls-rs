macro_rules! deps {
    () => {
        Injector!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Injector < T > { }
    };
}

impl_35!();