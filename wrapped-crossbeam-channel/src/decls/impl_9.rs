macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Sender < T > { }
    };
}

impl_9!()