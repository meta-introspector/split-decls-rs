macro_rules! deps {
    () => {
        Union!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < 'a , T , S > Iterator for Union < 'a , T , S > where T : Eq + Hash , S : BuildHasher , { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_180!();