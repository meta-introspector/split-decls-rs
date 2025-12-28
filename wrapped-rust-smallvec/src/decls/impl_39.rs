macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T , const N : usize > ExactSizeIterator for IntoIter < T , N > { }
    };
}

impl_39!()