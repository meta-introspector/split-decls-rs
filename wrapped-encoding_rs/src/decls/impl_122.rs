macro_rules! deps {
    () => {
        EncodingVisitor!();
        Encoding!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de > Deserialize < 'de > for & 'static Encoding { fn deserialize < D > (deserializer : D) -> Result < & 'static Encoding , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (EncodingVisitor) } }
    };
}

impl_122!()