macro_rules! deps {
    () => {
        Send!();
        FuturesUnordered!();
    };
}

macro_rules! impl_871 {
    () => {
        deps!();
        unsafe impl < Fut : Send + Sync > Sync for FuturesUnordered < Fut > { }
    };
}

impl_871!()