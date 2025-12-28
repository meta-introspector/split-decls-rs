macro_rules! deps {
    () => {
        PathspecDiffEntries!();
    };
}

macro_rules! impl_568 {
    () => {
        deps!();
        impl < 'list > FusedIterator for PathspecDiffEntries < 'list > { }
    };
}

impl_568!()