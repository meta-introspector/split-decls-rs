macro_rules! deps {
    () => {
        RawRwLock!();
        RwLock!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        unsafe impl < R : RawRwLock + Sync , T : ? Sized + Send + Sync > Sync for RwLock < R , T > { }
    };
}

impl_109!();