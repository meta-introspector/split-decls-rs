macro_rules! deps {
    () => {
        CapturesPatternIter!();
    };
}

macro_rules! impl_616 {
    () => {
        deps!();
        impl < 'a > ExactSizeIterator for CapturesPatternIter < 'a > { }
    };
}

impl_616!();