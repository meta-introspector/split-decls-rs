macro_rules! deps {
    () => {
        Masks!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl ExactSizeIterator for Masks { }
    };
}

impl_42!()