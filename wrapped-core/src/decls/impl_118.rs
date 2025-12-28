macro_rules! deps {
    () => {
        ComObjectInner!();
        ComObject!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        unsafe impl < T : ComObjectInner + Sync > Sync for ComObject < T > { }
    };
}

impl_118!();