macro_rules! deps {
    () => {
        Intersection!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < 'a , T , S , const N : usize > Iterator for Intersection < 'a , T , S , N > where S : BuildHasher , T : Eq + Hash , { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { loop { let elt = self . iter . next () ? ; if self . other . contains (elt) { return Some (elt) ; } } } }
    };
}

impl_153!()