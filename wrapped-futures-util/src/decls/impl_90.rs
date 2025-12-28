macro_rules! deps {
    () => {
        Send!();
        Inner!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        unsafe impl < Fut > Send for Inner < Fut > where Fut : Future + Send , Fut :: Output : Send + Sync , { }
    };
}

impl_90!()