macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T , const N : usize > DoubleEndedIterator for IntoIter < T , N > { fn next_back (& mut self) -> Option < Self :: Item > { self . deque . pop_back () } }
    };
}

impl_43!()