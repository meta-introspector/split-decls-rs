macro_rules! deps {
    () => {
        DominatedByIter!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl < 'a , N > Iterator for DominatedByIter < 'a , N > where N : 'a + Copy + Eq + Hash , { type Item = N ; fn next (& mut self) -> Option < Self :: Item > { for (dominator , dominated) in self . iter . by_ref () { if dominated == & self . node && dominated != dominator { return Some (* dominator) ; } } None } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_354!()