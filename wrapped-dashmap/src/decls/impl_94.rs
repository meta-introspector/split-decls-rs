macro_rules! deps {
    () => {
        DashMap!();
        DashMapVisitor!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 'de , K , V , S > Deserialize < 'de > for DashMap < K , V , S > where K : Deserialize < 'de > + Eq + Hash , V : Deserialize < 'de > , S : BuildHasher + Clone + Default , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_map (DashMapVisitor :: < K , V , S > :: new ()) } }
    };
}

impl_94!()