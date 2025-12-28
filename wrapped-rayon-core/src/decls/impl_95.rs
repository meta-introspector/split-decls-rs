macro_rules! deps {
    () => {
        LatchRef!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        unsafe impl < L : Sync > Sync for LatchRef < '_ , L > { }
    };
}

impl_95!()