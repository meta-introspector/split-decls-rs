macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < K , V , S , const N : usize > FromIterator < (K , V) > for IndexMap < K , V , S , N > where K : Eq + Hash , S : BuildHasher + Default , { fn from_iter < I > (iterable : I) -> Self where I : IntoIterator < Item = (K , V) > , { let mut map = Self :: default () ; map . extend (iterable) ; map } }
    };
}

impl_112!();