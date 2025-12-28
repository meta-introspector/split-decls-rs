macro_rules! deps {
    () => {
        ChainSeq!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < A , B > ChainSeq < A , B > { fn new (a : A , b : B) -> ChainSeq < A , B > where A : ExactSizeIterator , B : ExactSizeIterator < Item = A :: Item > , { ChainSeq { chain : a . chain (b) } } }
    };
}

impl_310!()