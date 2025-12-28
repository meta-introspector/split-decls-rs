macro_rules! deps {
    () => {
        SetMatchesIter!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'a > core :: iter :: FusedIterator for SetMatchesIter < 'a > { }
    };
}

impl_167!();