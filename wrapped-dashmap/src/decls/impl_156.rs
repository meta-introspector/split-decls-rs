macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < K , V , S > Default for DashMap < K , V , S > where K : Eq + Hash , S : Default + BuildHasher + Clone , { fn default () -> Self { Self :: with_hasher (Default :: default ()) } }
    };
}

impl_156!()