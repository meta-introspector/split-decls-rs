macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T : Default > Default for AtomicCell < T > { fn default () -> Self { Self :: new (T :: default ()) } }
    };
}

impl_39!()