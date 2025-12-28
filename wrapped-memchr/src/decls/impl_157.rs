macro_rules! deps {
    () => {
        TwoIter!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < 'a , 'h > core :: iter :: FusedIterator for TwoIter < 'a , 'h > { }
    };
}

impl_157!();