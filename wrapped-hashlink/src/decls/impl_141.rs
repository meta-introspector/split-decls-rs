macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < T , S > FromIterator < T > for LinkedHashSet < T , S > where T : Eq + Hash , S : BuildHasher + Default , { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> LinkedHashSet < T , S > { let mut set = LinkedHashSet :: with_hasher (Default :: default ()) ; set . extend (iter) ; set } }
    };
}

impl_141!();