macro_rules! deps {
    () => {
        UnboundedInner!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for UnboundedInner < T > { }
    };
}

impl_95!();