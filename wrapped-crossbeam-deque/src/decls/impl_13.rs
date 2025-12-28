macro_rules! deps {
    () => {
        Worker!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Worker < T > { }
    };
}

impl_13!()