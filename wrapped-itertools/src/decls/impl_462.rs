macro_rules! deps {
    () => {
        RcIter!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl < A , I > FusedIterator for RcIter < I > where I : FusedIterator < Item = A > { }
    };
}

impl_462!()