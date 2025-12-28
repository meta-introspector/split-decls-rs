macro_rules! deps {
    () => {
        IterBytes!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < 'a > FusedIterator for IterBytes < 'a > { }
    };
}

impl_161!();