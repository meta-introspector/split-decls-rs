macro_rules! deps {
    () => {
        CoreMap!();
        IndexMap!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < K , V , S , const N : usize > Default for IndexMap < K , V , S , N > where S : Default , { fn default () -> Self { const { assert ! (N > 1) ; assert ! (N . is_power_of_two ()) ; } Self { build_hasher : < _ > :: default () , core : CoreMap :: new () , } } }
    };
}

impl_107!()