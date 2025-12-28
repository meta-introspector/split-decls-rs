macro_rules! deps {
    () => {
        Atomic!();
        Pointable!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Pointable + Send + Sync > Sync for Atomic < T > { }
    };
}

impl_20!()