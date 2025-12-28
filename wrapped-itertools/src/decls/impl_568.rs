macro_rules! deps {
    () => {
        WithPosition!();
    };
}

macro_rules! impl_568 {
    () => {
        deps!();
        impl < I : Iterator > FusedIterator for WithPosition < I > { }
    };
}

impl_568!()