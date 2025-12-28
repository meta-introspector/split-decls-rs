macro_rules! deps {
    () => {
        ChainSeq!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl < A , B > DoubleEndedIterator for ChainSeq < A , B > where A : DoubleEndedIterator , B : DoubleEndedIterator < Item = A :: Item > , { fn next_back (& mut self) -> Option < Self :: Item > { self . chain . next_back () } }
    };
}

impl_313!();