macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < 'a , T , S > Iterator for Difference < 'a , T , S > where T : Eq + Hash , S : BuildHasher , { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { loop { match self . iter . next () { None => return None , Some (elt) => { if ! self . other . contains (elt) { return Some (elt) ; } } } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_173!()