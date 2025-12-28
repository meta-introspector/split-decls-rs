macro_rules! deps {
    () => {
        DominatorsIter!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl < 'a , N > Iterator for DominatorsIter < 'a , N > where N : 'a + Copy + Eq + Hash , { type Item = N ; fn next (& mut self) -> Option < Self :: Item > { let next = self . node . take () ; if let Some (next) = next { self . node = self . dominators . immediate_dominator (next) ; } next } }
    };
}

impl_352!();