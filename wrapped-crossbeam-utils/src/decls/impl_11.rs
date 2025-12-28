macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for AtomicCell < T > { }
    };
}

impl_11!()