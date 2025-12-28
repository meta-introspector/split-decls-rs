macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl RawStream for Vec < u8 > { }
    };
}

impl_53!();