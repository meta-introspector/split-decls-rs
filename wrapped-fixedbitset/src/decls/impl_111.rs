macro_rules! deps {
    () => {
        Masks!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl ExactSizeIterator for Masks { }
    };
}

impl_111!()