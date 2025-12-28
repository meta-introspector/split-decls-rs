macro_rules! deps {
    () => {
        Intersection!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < 'a , T , S > Iterator for Intersection < 'a , T , S > where T : Eq + Hash , S : BuildHasher , { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { loop { match self . iter . next () { None => return None , Some (elt) => { if self . other . contains (elt) { return Some (elt) ; } } } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_170!()