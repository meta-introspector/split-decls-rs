macro_rules! deps {
    () => {
        Primitive!();
        AtomicMaybeUninit!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < T : Primitive > core :: panic :: RefUnwindSafe for AtomicMaybeUninit < T > { }
    };
}

impl_97!();