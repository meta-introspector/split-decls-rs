macro_rules! deps {
    () => {
        AtomicMaybeUninit!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T : Primitive > core :: panic :: RefUnwindSafe for AtomicMaybeUninit < T > { }
    };
}

impl_10!()