macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < 'a , K , V , S > Extend < (& 'a K , & 'a V) > for IndexMap < K , V , S > where K : Hash + Eq + Copy , V : Copy , S : BuildHasher , { # [doc = " Extend the map with all key-value pairs in the iterable."] # [doc = ""] # [doc = " See the first extend method for more details."] fn extend < I : IntoIterator < Item = (& 'a K , & 'a V) > > (& mut self , iterable : I) { self . extend (iterable . into_iter () . map (| (& key , & value) | (key , value))) ; } }
    };
}

impl_63!()