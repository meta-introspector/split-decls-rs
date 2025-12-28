macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < K , V , S > FromIterator < (K , V) > for IndexMap < K , V , S > where K : Hash + Eq , S : BuildHasher + Default , { # [doc = " Create an `IndexMap` from the sequence of key-value pairs in the"] # [doc = " iterable."] # [doc = ""] # [doc = " `from_iter` uses the same logic as `extend`. See"] # [doc = " [`extend`][IndexMap::extend] for more details."] fn from_iter < I : IntoIterator < Item = (K , V) > > (iterable : I) -> Self { let iter = iterable . into_iter () ; let (low , _) = iter . size_hint () ; let mut map = Self :: with_capacity_and_hasher (low , < _ > :: default ()) ; map . extend (iter) ; map } }
    };
}

impl_60!();