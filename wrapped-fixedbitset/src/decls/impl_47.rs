macro_rules! deps {
    () => {
        Ones!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a > FusedIterator for Ones < 'a > { }
    };
}

impl_47!()