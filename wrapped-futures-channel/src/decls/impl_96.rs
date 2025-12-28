macro_rules! deps {
    () => {
        BoundedInner!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for BoundedInner < T > { }
    };
}

impl_96!()