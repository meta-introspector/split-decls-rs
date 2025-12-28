macro_rules! deps {
    () => {
        RawMutex!();
        Mutex!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        unsafe impl < R : RawMutex + Sync , T : ? Sized + Send > Sync for Mutex < R , T > { }
    };
}

impl_10!();