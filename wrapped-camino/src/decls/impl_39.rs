macro_rules! deps {
    () => {
        Utf8Components!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl FusedIterator for Utf8Components < '_ > { }
    };
}

impl_39!()