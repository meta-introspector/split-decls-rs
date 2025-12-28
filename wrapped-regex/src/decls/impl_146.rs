macro_rules! deps {
    () => {
        SetMatchesIntoIter!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl core :: iter :: FusedIterator for SetMatchesIntoIter { }
    };
}

impl_146!()