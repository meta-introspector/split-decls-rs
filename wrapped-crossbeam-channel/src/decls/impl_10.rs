macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for Sender < T > { }
    };
}

impl_10!()