macro_rules! deps {
    () => {
        PathspecEntries!();
    };
}

macro_rules! impl_564 {
    () => {
        deps!();
        impl < 'list > FusedIterator for PathspecEntries < 'list > { }
    };
}

impl_564!()