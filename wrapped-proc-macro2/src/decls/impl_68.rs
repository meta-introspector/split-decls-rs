macro_rules! deps {
    () => {
        RcVecIntoIter!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T > Iterator for RcVecIntoIter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_68!()