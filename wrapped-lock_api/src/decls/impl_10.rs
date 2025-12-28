macro_rules! deps {
    () => {
        Mutex!();
        RawMutex!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        unsafe impl < R : RawMutex + Sync , T : ? Sized + Send > Sync for Mutex < R , T > { }
    };
}

impl_10!()