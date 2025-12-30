// Generated macro for impl_618 (impl)
macro_rules! Depcrateimpl_618 {
() => {
// Module: crate
// Provides: {"impl_618"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > Deserialize < 'de > for & 'static Encoding { fn deserialize < D > (deserializer : D) -> Result < & 'static Encoding , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (EncodingVisitor) } }
};
}
