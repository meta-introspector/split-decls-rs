macro_rules! deps {
    () => {
        FullBucketsIndices!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl ExactSizeIterator for FullBucketsIndices { }
    };
}

impl_85!();