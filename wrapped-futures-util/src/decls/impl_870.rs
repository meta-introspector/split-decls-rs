macro_rules! deps {
    () => {
        Send!();
        FuturesUnordered!();
    };
}

macro_rules! impl_870 {
    () => {
        deps!();
        unsafe impl < Fut : Send > Send for FuturesUnordered < Fut > { }
    };
}

impl_870!()