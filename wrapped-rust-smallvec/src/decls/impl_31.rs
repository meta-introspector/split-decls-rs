macro_rules! deps {
    () => {
        Splice!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < I : Iterator , const N : usize > ExactSizeIterator for Splice < '_ , I , N > { }
    };
}

impl_31!()