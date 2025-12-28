macro_rules! deps {
    () => {
        OdbReader!();
    };
}

macro_rules! impl_497 {
    () => {
        deps!();
        unsafe impl < 'repo > Send for OdbReader < 'repo > { }
    };
}

impl_497!();