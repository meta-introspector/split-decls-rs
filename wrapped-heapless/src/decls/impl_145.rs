macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < T , S , const N : usize > FromIterator < T > for IndexSet < T , S , N > where T : Eq + Hash , S : BuildHasher + Default , { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { let mut set = Self :: default () ; set . extend (iter) ; set } }
    };
}

impl_145!();