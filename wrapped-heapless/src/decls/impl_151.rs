macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < 'a , T , S , const N : usize > Iterator for Difference < 'a , T , S , N > where S : BuildHasher , T : Eq + Hash , { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { loop { let elt = self . iter . next () ? ; if ! self . other . contains (elt) { return Some (elt) ; } } } }
    };
}

impl_151!();