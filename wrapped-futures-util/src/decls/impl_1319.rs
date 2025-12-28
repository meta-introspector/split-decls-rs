macro_rules! deps {
    () => {
        Send!();
        Mutex!();
    };
}

macro_rules! impl_1319 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Sync for Mutex < T > { }
    };
}

impl_1319!();