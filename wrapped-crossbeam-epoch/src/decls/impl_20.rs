macro_rules! deps {
    () => {
        Pointable!();
        Atomic!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Pointable + Send + Sync > Sync for Atomic < T > { }
    };
}

impl_20!();