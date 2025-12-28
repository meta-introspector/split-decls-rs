macro_rules! deps {
    () => {
        IterEither!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < L , R > iter :: FusedIterator for IterEither < L , R > where L : iter :: FusedIterator , R : iter :: FusedIterator , { }
    };
}

impl_25!();