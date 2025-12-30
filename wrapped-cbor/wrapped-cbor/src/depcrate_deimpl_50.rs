// Generated macro for impl_50 (impl)
macro_rules! Depcrate_deimpl_50 {
() => {
// Module: crate::de
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'de , V > de :: DeserializeSeed < 'de > for StructVariantSeed < V > where V : de :: Visitor < 'de > , { type Value = V :: Value ; fn deserialize < D > (self , de : D) -> result :: Result < V :: Value , D :: Error > where D : de :: Deserializer < 'de > , { de . deserialize_any (self . visitor) } }
};
}
