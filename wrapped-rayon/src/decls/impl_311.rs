macro_rules! deps {
    () => {
        ChainSeq!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl < A , B > Iterator for ChainSeq < A , B > where A : Iterator , B : Iterator < Item = A :: Item > , { type Item = A :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . chain . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . chain . size_hint () } }
    };
}

impl_311!()