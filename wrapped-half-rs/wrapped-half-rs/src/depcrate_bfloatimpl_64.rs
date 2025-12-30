// Generated macro for impl_64 (impl)
macro_rules! Depcrate_bfloatimpl_64 {
() => {
// Module: crate::bfloat
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > Deserialize < 'de > for bf16 { fn deserialize < D > (deserializer : D) -> Result < bf16 , D :: Error > where D : serde :: de :: Deserializer < 'de > , { deserializer . deserialize_newtype_struct ("bf16" , Visitor) } }
};
}
