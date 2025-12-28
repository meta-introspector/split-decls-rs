macro_rules! deps {
    () => {
        Zeroes!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'a > FusedIterator for Zeroes < 'a > { }
    };
}

impl_119!()