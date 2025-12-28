macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < K , V , S , const N : usize > Eq for IndexMap < K , V , S , N > where K : Eq + Hash , V : Eq , S : BuildHasher , { }
    };
}

impl_109!()