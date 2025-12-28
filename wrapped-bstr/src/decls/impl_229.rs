macro_rules! deps {
    () => {
        CharIndices!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < 'a > :: core :: iter :: FusedIterator for CharIndices < 'a > { }
    };
}

impl_229!();