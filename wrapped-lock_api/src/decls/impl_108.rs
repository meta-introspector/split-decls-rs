macro_rules! deps {
    () => {
        RawRwLock!();
        RwLock!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        unsafe impl < R : RawRwLock + Send , T : ? Sized + Send > Send for RwLock < R , T > { }
    };
}

impl_108!()