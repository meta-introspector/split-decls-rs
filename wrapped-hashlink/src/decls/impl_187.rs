macro_rules! deps {
    () => {
        LruCache!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < K : Eq + Hash , V , S : BuildHasher > Extend < (K , V) > for LruCache < K , V , S > { # [inline] fn extend < I : IntoIterator < Item = (K , V) > > (& mut self , iter : I) { for (k , v) in iter { self . insert (k , v) ; } } }
    };
}

impl_187!();