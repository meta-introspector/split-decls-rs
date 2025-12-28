macro_rules! deps {
    () => {
        Lines!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 'a > iter :: FusedIterator for Lines < 'a > { }
    };
}

impl_104!();