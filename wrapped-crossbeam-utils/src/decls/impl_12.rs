macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for AtomicCell < T > { }
    };
}

impl_12!()