macro_rules! deps {
    () => {
        ComObject!();
        ComObjectInner!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < T : ComObjectInner + Eq > Eq for ComObject < T > { }
    };
}

impl_120!()