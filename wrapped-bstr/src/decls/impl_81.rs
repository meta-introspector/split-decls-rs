macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < 'a > iter :: FusedIterator for Bytes < 'a > { }
    };
}

impl_81!()