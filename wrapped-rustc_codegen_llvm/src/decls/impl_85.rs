macro_rules! deps {
    () => {
        ThinData!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        unsafe impl Send for ThinData { }
    };
}

impl_85!()