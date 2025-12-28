macro_rules! deps {
    () => {
        Utf8Ancestors!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl FusedIterator for Utf8Ancestors < '_ > { }
    };
}

impl_18!()