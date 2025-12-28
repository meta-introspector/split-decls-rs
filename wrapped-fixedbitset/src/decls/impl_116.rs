macro_rules! deps {
    () => {
        Ones!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < 'a > FusedIterator for Ones < 'a > { }
    };
}

impl_116!();