macro_rules! deps {
    () => {
        LinearMap!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < K , V , const N : usize > FromIterator < (K , V) > for LinearMap < K , V , N > where K : Eq , { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , V) > , { let mut out = Self :: new () ; out . buffer . extend (iter) ; out } }
    };
}

impl_176!()