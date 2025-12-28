macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T , const N : usize > ExactSizeIterator for IntoIter < T , N > { fn len (& self) -> usize { self . deque . len () } }
    };
}

impl_45!();