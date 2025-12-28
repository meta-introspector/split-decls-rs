macro_rules! deps {
    () => {
        UnboundedInner!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for UnboundedInner < T > { }
    };
}

impl_94!()