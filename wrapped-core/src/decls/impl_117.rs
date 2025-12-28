macro_rules! deps {
    () => {
        ComObject!();
        ComObjectInner!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        unsafe impl < T : ComObjectInner + Send > Send for ComObject < T > { }
    };
}

impl_117!()