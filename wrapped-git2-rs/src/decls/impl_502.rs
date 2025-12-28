macro_rules! deps {
    () => {
        OdbWriter!();
    };
}

macro_rules! impl_502 {
    () => {
        deps!();
        unsafe impl < 'repo > Send for OdbWriter < 'repo > { }
    };
}

impl_502!()