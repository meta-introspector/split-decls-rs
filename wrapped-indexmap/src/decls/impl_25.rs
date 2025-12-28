macro_rules! deps {
    () => {
        IndexMapVisitor!();
        IndexMap!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'de , K , V , S > Deserialize < 'de > for IndexMap < K , V , S > where K : Deserialize < 'de > + Eq + Hash , V : Deserialize < 'de > , S : Default + BuildHasher , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_map (IndexMapVisitor (PhantomData)) } }
    };
}

impl_25!();