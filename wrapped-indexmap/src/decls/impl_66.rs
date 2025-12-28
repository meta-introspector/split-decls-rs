macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < K , V , S > Eq for IndexMap < K , V , S > where K : Eq + Hash , V : Eq , S : BuildHasher , { }
    };
}

impl_66!();