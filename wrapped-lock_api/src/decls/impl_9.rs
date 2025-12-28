macro_rules! deps {
    () => {
        Mutex!();
        RawMutex!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        unsafe impl < R : RawMutex + Send , T : ? Sized + Send > Send for Mutex < R , T > { }
    };
}

impl_9!();