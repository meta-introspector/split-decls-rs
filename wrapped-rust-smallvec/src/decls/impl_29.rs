macro_rules! deps {
    () => {
        Splice!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < I : Iterator , const N : usize > Iterator for Splice < '_ , I , N > { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . drain . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . drain . size_hint () } }
    };
}

impl_29!()