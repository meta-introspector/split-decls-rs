macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < K : Hash + Eq , V , S : BuildHasher + Default > FromIterator < (K , V) > for LinkedHashMap < K , V , S > { # [inline] fn from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self { let iter = iter . into_iter () ; let mut map = Self :: with_capacity_and_hasher (iter . size_hint () . 0 , S :: default ()) ; map . extend (iter) ; map } }
    };
}

impl_8!()