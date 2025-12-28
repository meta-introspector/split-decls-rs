macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < K , V , S , const N : usize > Clone for IndexMap < K , V , S , N > where K : Clone , V : Clone , S : Clone , { fn clone (& self) -> Self { Self { core : self . core . clone () , build_hasher : self . build_hasher . clone () , } } }
    };
}

impl_105!()