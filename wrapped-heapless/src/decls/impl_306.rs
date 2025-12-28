macro_rules! deps {
    () => {
        LenType!();
        IntoIter!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > FusedIterator for IntoIter < T , N , LenT > { }
    };
}

impl_306!();