macro_rules! deps {
    () => {
        Inner!();
        Send!();
    };
}

macro_rules! impl_1261 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for Inner < T > { }
    };
}

impl_1261!();