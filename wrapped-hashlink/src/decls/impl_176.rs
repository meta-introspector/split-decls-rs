macro_rules! deps {
    () => {
        SymmetricDifference!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < 'a , T , S > Iterator for SymmetricDifference < 'a , T , S > where T : Eq + Hash , S : BuildHasher , { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_176!();