macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'a , K , V , S > Extend < (& 'a K , & 'a V) > for LinkedHashMap < K , V , S > where K : 'a + Hash + Eq + Copy , V : 'a + Copy , S : BuildHasher , { # [inline] fn extend < I : IntoIterator < Item = (& 'a K , & 'a V) > > (& mut self , iter : I) { for (& k , & v) in iter { self . insert (k , v) ; } } }
    };
}

impl_22!()