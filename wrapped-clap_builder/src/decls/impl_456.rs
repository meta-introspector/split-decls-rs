macro_rules! deps {
    () => {
        ValuesRef!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl < 'a , T : 'a > ExactSizeIterator for ValuesRef < 'a , T > { }
    };
}

impl_456!();