macro_rules! deps {
    () => {
        RawMutex!();
        Mutex!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        unsafe impl < R : RawMutex + Send , T : ? Sized + Send > Send for Mutex < R , T > { }
    };
}

impl_9!()