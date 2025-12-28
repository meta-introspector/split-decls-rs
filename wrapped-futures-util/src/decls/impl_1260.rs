macro_rules! deps {
    () => {
        Inner!();
        Send!();
    };
}

macro_rules! impl_1260 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Inner < T > { }
    };
}

impl_1260!()