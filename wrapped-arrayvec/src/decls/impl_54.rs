macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < T , const CAP : usize > ExactSizeIterator for IntoIter < T , CAP > { }
    };
}

impl_54!()