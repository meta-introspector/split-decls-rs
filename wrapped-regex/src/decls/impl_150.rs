macro_rules! deps {
    () => {
        SetMatchesIter!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < 'a > core :: iter :: FusedIterator for SetMatchesIter < 'a > { }
    };
}

impl_150!();