// Generated macro for impl_80 (impl)
macro_rules! Depcrate_de_valueimpl_80 {
() => {
// Module: crate::de::value
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Value { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_any (ValueVisitor) } }
};
}
