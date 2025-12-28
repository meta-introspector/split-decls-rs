macro_rules! deps {
    () => {
        Deltas!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl < 'diff > FusedIterator for Deltas < 'diff > { }
    };
}

impl_340!();