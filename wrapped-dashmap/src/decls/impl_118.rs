macro_rules! deps {
    () => {
        DashSet!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < K : Eq + Hash , S : BuildHasher + Clone > Eq for DashSet < K , S > { }
    };
}

impl_118!();