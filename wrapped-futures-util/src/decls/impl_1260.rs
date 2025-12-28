macro_rules! deps {
    () => {
        Send!();
        Inner!();
    };
}

macro_rules! impl_1260 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Inner < T > { }
    };
}

impl_1260!();