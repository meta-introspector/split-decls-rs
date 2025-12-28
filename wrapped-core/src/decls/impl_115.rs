macro_rules! deps {
    () => {
        ComObject!();
        ComObjectInner!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < T : ComObjectInner > From < T > for ComObject < T > { fn from (value : T) -> Self { Self :: new (value) } }
    };
}

impl_115!()