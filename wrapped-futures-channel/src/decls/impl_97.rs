macro_rules! deps {
    () => {
        BoundedInner!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for BoundedInner < T > { }
    };
}

impl_97!();