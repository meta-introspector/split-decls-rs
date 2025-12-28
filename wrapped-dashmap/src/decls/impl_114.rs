macro_rules! deps {
    () => {
        DashSet!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < K , S > Default for DashSet < K , S > where K : Eq + Hash , S : Default + BuildHasher + Clone , { fn default () -> Self { Self :: with_hasher (Default :: default ()) } }
    };
}

impl_114!();