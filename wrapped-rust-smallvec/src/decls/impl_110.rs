macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < T , const N : usize > core :: iter :: FusedIterator for IntoIter < T , N > { }
    };
}

impl_110!()