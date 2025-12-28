macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Receiver < T > { }
    };
}

impl_19!()