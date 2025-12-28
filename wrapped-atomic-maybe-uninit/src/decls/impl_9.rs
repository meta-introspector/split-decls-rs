macro_rules! deps {
    () => {
        AtomicMaybeUninit!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        unsafe impl < T : Primitive > Sync for AtomicMaybeUninit < T > { }
    };
}

impl_9!()