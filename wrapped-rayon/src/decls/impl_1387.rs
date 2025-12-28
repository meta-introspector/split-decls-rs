macro_rules! deps {
    () => {
        SendPtr!();
    };
}

macro_rules! impl_1387 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for SendPtr < T > { }
    };
}

impl_1387!()