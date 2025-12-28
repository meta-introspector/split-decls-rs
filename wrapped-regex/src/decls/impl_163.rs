macro_rules! deps {
    () => {
        SetMatchesIntoIter!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl core :: iter :: FusedIterator for SetMatchesIntoIter { }
    };
}

impl_163!();