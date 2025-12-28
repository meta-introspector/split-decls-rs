macro_rules! deps {
    () => {
        ComObjectInner!();
        ComObject!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < T : ComObjectInner + Eq > Eq for ComObject < T > { }
    };
}

impl_120!();