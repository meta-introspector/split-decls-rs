macro_rules! deps {
    () => {
        StoreFromIterable!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < K , V , S > FromIterator < (K , V) > for LiteMap < K , V , S > where K : Ord , S : StoreFromIterable < K , V > , { fn from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self { let values = S :: lm_sort_from_iter (iter) ; Self :: from_sorted_store_unchecked (values) } }
    };
}

impl_20!()