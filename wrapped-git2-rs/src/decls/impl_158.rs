macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < 'a > ExactSizeIterator for Iter < 'a > { }
    };
}

impl_158!();