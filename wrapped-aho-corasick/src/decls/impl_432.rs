macro_rules! deps {
    () => {
        IteratorIndexExt!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl < I : Iterator > IteratorIndexExt for I { }
    };
}

impl_432!();