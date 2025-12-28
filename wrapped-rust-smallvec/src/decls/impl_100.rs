macro_rules! deps {
    () => {
        Splice!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < I : Iterator , const N : usize > DoubleEndedIterator for Splice < '_ , I , N > { fn next_back (& mut self) -> Option < Self :: Item > { self . drain . next_back () } }
    };
}

impl_100!()