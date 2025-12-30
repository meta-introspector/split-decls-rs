// Generated macro for impl_381 (impl)
macro_rules! Depcrate_value_deimpl_381 {
() => {
// Module: crate::value::de
// Provides: {"impl_381"}
// Dependencies: {}
impl < 'de > DeserializeSeed < 'de > for KeyClassifier { type Value = KeyClass ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: Deserializer < 'de > , { deserializer . deserialize_str (self) } }
};
}
