macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for Receiver < T > { }
    };
}

impl_20!()