macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < L , R > iter :: FusedIterator for Either < L , R > where L : iter :: FusedIterator , R : iter :: FusedIterator < Item = L :: Item > , { }
    };
}

impl_21!();