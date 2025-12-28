macro_rules! deps {
    () => {
        IdsRef!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl ExactSizeIterator for IdsRef < '_ > { }
    };
}

impl_447!()