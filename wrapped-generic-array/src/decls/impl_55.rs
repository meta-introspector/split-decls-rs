macro_rules! deps {
    () => {
        GenericArrayIter!();
        ArrayLength!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < T , N : ArrayLength > ExactSizeIterator for GenericArrayIter < T , N > { # [inline] fn len (& self) -> usize { self . index_back - self . index } }
    };
}

impl_55!()