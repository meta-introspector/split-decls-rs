macro_rules! deps {
    () => {
        SendPtr!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for SendPtr < T > { }
    };
}

impl_20!()