macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T , const N : usize > Iterator for IntoIter < T , N > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . deque . pop_front () } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
    };
}

impl_42!()