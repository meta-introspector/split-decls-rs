macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V , S : BuildHasher + Clone > IntoIterator for & 'a DashMap < K , V , S > { type Item = RefMulti < 'a , K , V > ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_31!()