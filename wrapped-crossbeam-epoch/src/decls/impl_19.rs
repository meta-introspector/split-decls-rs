macro_rules! deps {
    () => {
        Atomic!();
        Pointable!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Pointable + Send + Sync > Send for Atomic < T > { }
    };
}

impl_19!();