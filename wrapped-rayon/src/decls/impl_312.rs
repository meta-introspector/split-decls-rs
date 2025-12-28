macro_rules! deps {
    () => {
        ChainSeq!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < A , B > ExactSizeIterator for ChainSeq < A , B > where A : ExactSizeIterator , B : ExactSizeIterator < Item = A :: Item > , { }
    };
}

impl_312!()