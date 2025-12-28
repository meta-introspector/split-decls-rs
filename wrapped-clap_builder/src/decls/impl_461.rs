macro_rules! deps {
    () => {
        RawValues!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl ExactSizeIterator for RawValues < '_ > { }
    };
}

impl_461!()