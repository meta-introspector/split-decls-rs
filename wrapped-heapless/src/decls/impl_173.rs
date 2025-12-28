macro_rules! deps {
    () => {
        LinearMap!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < K , V , const N : usize > Default for LinearMap < K , V , N > where K : Eq , { fn default () -> Self { Self :: new () } }
    };
}

impl_173!()