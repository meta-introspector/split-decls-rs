macro_rules! deps {
    () => {
        FullBucketsIndices!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl FusedIterator for FullBucketsIndices { }
    };
}

impl_86!();