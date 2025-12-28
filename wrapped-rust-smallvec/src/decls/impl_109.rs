macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < T , const N : usize > ExactSizeIterator for IntoIter < T , N > { }
    };
}

impl_109!();