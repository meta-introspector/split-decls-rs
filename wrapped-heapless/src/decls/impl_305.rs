macro_rules! deps {
    () => {
        IntoIter!();
        LenType!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > DoubleEndedIterator for IntoIter < T , N , LenT > { fn next_back (& mut self) -> Option < Self :: Item > { if self . next < self . vec . len { let item = unsafe { self . vec . pop_unchecked () } ; Some (item) } else { None } } }
    };
}

impl_305!()