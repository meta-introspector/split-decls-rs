macro_rules! deps {
    () => {
        RwLock!();
        RawRwLock!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        unsafe impl < R : RawRwLock + Send , T : ? Sized + Send > Send for RwLock < R , T > { }
    };
}

impl_108!();