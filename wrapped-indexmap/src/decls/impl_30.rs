macro_rules! deps {
    () => {
        IndexSetVisitor!();
        IndexSet!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < 'de , T , S > Deserialize < 'de > for IndexSet < T , S > where T : Deserialize < 'de > + Eq + Hash , S : Default + BuildHasher , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_seq (IndexSetVisitor (PhantomData)) } }
    };
}

impl_30!();