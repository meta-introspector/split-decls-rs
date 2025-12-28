macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < K : Hash + Eq , V , S : BuildHasher > Extend < (K , V) > for LinkedHashMap < K , V , S > { # [inline] fn extend < I : IntoIterator < Item = (K , V) > > (& mut self , iter : I) { for (k , v) in iter { self . insert (k , v) ; } } }
    };
}

impl_21!()