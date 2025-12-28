macro_rules! deps {
    () => {
        IntoIter!();
        LenType!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > FusedIterator for IntoIter < T , N , LenT > { }
    };
}

impl_306!()