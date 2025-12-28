macro_rules! deps {
    () => {
        StoreBulkMut!();
        LiteMapVisitor!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < 'de , K , V , R > Deserialize < 'de > for LiteMap < K , V , R > where K : Ord + Deserialize < 'de > , V : Deserialize < 'de > , R : StoreBulkMut < K , V > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { deserializer . deserialize_any (LiteMapVisitor :: new ()) } else { deserializer . deserialize_map (LiteMapVisitor :: new ()) } } }
    };
}

impl_61!()