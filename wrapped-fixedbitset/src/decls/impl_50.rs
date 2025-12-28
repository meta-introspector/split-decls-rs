macro_rules! deps {
    () => {
        Zeroes!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < 'a > FusedIterator for Zeroes < 'a > { }
    };
}

impl_50!()