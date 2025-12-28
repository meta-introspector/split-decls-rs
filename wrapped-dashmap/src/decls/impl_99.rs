macro_rules! deps {
    () => {
        DashSetVisitor!();
        DashSet!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < 'de , K , S > Deserialize < 'de > for DashSet < K , S > where K : Deserialize < 'de > + Eq + Hash , S : BuildHasher + Clone + Default , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_seq (DashSetVisitor :: < K , S > :: new ()) } }
    };
}

impl_99!();