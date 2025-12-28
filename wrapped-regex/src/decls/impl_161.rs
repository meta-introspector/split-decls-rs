macro_rules! deps {
    () => {
        SetMatchesIntoIter!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl Iterator for SetMatchesIntoIter { type Item = usize ; fn next (& mut self) -> Option < usize > { loop { let id = self . it . next () ? ; if self . patset . contains (PatternID :: new_unchecked (id)) { return Some (id) ; } } } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_161!()