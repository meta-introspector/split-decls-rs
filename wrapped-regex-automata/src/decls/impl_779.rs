macro_rules! deps {
    () => {
        IteratorIndexExt!();
    };
}

macro_rules! impl_779 {
    () => {
        deps!();
        impl < I : Iterator > IteratorIndexExt for I { }
    };
}

impl_779!();