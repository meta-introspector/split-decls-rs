// Generated macro for impl_13 (impl)
macro_rules! Depcrate_serde_implsimpl_13 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Utf8PathBuf { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_string (Utf8PathBufVisitor) } }
};
}
