macro_rules! deps {
    () => {
        CapturesPatternIter!();
    };
}

macro_rules! impl_617 {
    () => {
        deps!();
        impl < 'a > core :: iter :: FusedIterator for CapturesPatternIter < 'a > { }
    };
}

impl_617!();