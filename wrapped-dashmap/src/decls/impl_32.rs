macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < K : Eq + Hash , V , S : BuildHasher + Clone > Extend < (K , V) > for DashMap < K , V , S > { fn extend < I : IntoIterator < Item = (K , V) > > (& mut self , intoiter : I) { for pair in intoiter . into_iter () { self . insert (pair . 0 , pair . 1) ; } } }
    };
}

impl_32!()