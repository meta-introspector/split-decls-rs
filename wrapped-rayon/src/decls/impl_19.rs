macro_rules! deps {
    () => {
        SendPtr!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for SendPtr < T > { }
    };
}

impl_19!()