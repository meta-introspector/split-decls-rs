macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < 'a > FusedIterator for Iter < 'a > { }
    };
}

impl_157!()