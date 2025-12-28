macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'a , K : 'a + Eq + Hash , V : 'a + Eq , S : BuildHasher + Clone > Eq for DashMap < K , V , S > { }
    };
}

impl_167!();