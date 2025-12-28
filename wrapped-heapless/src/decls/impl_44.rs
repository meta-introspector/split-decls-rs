macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T , const N : usize > FusedIterator for IntoIter < T , N > { }
    };
}

impl_44!();