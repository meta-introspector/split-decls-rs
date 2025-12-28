macro_rules! deps {
    () => {
        Mutex!();
        Send!();
    };
}

macro_rules! impl_1319 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Sync for Mutex < T > { }
    };
}

impl_1319!()