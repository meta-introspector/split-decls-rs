macro_rules! deps {
    () => {
        Indices!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl ExactSizeIterator for Indices < '_ > { }
    };
}

impl_493!()