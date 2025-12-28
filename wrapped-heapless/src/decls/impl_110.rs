macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < K , V , S , const N : usize > Extend < (K , V) > for IndexMap < K , V , S , N > where K : Eq + Hash , S : BuildHasher , { fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = (K , V) > , { for (k , v) in iterable { self . insert (k , v) . ok () . unwrap () ; } } }
    };
}

impl_110!();