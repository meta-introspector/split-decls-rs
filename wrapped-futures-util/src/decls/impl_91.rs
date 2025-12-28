macro_rules! deps {
    () => {
        Inner!();
        Send!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        unsafe impl < Fut > Sync for Inner < Fut > where Fut : Future + Send , Fut :: Output : Send + Sync , { }
    };
}

impl_91!()