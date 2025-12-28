macro_rules! deps {
    () => {
        StoreFromIterable!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < K : Ord , V > StoreFromIterable < K , V > for Vec < (K , V) > { fn lm_sort_from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self { let mut v = Self :: new () ; v . lm_extend (iter) ; v } }
    };
}

impl_83!();