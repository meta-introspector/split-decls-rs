macro_rules! deps {
    () => {
        SetMatchesIntoIter!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl DoubleEndedIterator for SetMatchesIntoIter { fn next_back (& mut self) -> Option < usize > { loop { let id = self . it . next_back () ? ; if self . patset . contains (PatternID :: new_unchecked (id)) { return Some (id) ; } } } }
    };
}

impl_162!()