macro_rules! deps {
    () => {
        Send!();
        Mutex!();
    };
}

macro_rules! impl_1318 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Send for Mutex < T > { }
    };
}

impl_1318!()