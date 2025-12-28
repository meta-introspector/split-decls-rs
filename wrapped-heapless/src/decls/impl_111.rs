macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < 'a , K , V , S , const N : usize > Extend < (& 'a K , & 'a V) > for IndexMap < K , V , S , N > where K : Eq + Hash + Copy , V : Copy , S : BuildHasher , { fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = (& 'a K , & 'a V) > , { self . extend (iterable . into_iter () . map (| (& key , & value) | (key , value))) ; } }
    };
}

impl_111!()