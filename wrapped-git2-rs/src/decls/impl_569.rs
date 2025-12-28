macro_rules! deps {
    () => {
        PathspecDiffEntries!();
    };
}

macro_rules! impl_569 {
    () => {
        deps!();
        impl < 'list > ExactSizeIterator for PathspecDiffEntries < 'list > { }
    };
}

impl_569!()