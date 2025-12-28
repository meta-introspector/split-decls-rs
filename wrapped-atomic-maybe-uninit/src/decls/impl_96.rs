macro_rules! deps {
    () => {
        Primitive!();
        AtomicMaybeUninit!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        unsafe impl < T : Primitive > Sync for AtomicMaybeUninit < T > { }
    };
}

impl_96!()