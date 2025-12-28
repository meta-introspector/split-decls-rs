macro_rules! deps {
    () => {
        Utf8Ancestors!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl FusedIterator for Utf8Ancestors < '_ > { }
    };
}

impl_35!();