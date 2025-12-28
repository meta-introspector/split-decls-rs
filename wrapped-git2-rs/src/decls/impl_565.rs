macro_rules! deps {
    () => {
        PathspecEntries!();
    };
}

macro_rules! impl_565 {
    () => {
        deps!();
        impl < 'list > ExactSizeIterator for PathspecEntries < 'list > { }
    };
}

impl_565!();