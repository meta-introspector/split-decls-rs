macro_rules! deps {
    () => {
        CLruCache!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < K : Clone + Eq + Hash , V , S : BuildHasher > Extend < (K , V) > for CLruCache < K , V , S > { fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { for (k , v) in iter { self . put (k , v) ; } } }
    };
}

impl_28!()