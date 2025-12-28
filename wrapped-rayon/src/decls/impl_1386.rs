macro_rules! deps {
    () => {
        SendPtr!();
    };
}

macro_rules! impl_1386 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for SendPtr < T > { }
    };
}

impl_1386!()