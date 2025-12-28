macro_rules! deps {
    () => {
        ComObject!();
        ComObjectInner!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < T : ComObjectInner + Default > Default for ComObject < T > { fn default () -> Self { Self :: new (T :: default ()) } }
    };
}

impl_110!()