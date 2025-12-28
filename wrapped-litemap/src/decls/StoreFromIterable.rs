macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! StoreFromIterable {
    () => {
        deps!();
        pub trait StoreFromIterable < K , V > : Store < K , V > { # [doc = " Create a sorted store from `iter`."] fn lm_sort_from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self ; }
    };
}

StoreFromIterable!();