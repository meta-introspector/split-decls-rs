macro_rules! deps {
    () => {
        Scalar!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        impl Eq for Scalar { }
    };
}

impl_375!()