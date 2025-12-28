macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < K : Eq + Hash , V , S : BuildHasher + Clone > IntoIterator for DashMap < K , V , S > { type Item = (K , V) ; type IntoIter = OwningIter < K , V > ; fn into_iter (self) -> Self :: IntoIter { OwningIter :: new (self) } }
    };
}

impl_30!()