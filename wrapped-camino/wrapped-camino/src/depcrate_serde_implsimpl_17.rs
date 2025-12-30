// Generated macro for impl_17 (impl)
macro_rules! Depcrate_serde_implsimpl_17 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'de : 'a , 'a > Deserialize < 'de > for & 'a Utf8Path { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (Utf8PathVisitor) } }
};
}
