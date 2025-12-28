macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T , const N : usize > core :: iter :: FusedIterator for IntoIter < T , N > { }
    };
}

impl_40!()