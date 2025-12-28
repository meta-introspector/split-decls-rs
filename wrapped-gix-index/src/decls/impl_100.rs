macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Self { trust_ctime : true , check_stat : true , use_nsec : false , use_stdev : false , } } }
    };
}

impl_100!()