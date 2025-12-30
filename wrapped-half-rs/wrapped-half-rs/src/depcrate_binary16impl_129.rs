// Generated macro for impl_129 (impl)
macro_rules! Depcrate_binary16impl_129 {
() => {
// Module: crate::binary16
// Provides: {"impl_129"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > Deserialize < 'de > for f16 { fn deserialize < D > (deserializer : D) -> Result < f16 , D :: Error > where D : serde :: de :: Deserializer < 'de > , { deserializer . deserialize_newtype_struct ("f16" , Visitor) } }
};
}
