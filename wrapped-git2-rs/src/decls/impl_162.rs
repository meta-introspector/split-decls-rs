macro_rules! deps {
    () => {
        IterBytes!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < 'a > ExactSizeIterator for IterBytes < 'a > { }
    };
}

impl_162!();