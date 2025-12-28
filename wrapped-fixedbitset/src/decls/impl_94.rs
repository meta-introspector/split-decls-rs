macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 'a > FusedIterator for Difference < 'a > { }
    };
}

impl_94!()