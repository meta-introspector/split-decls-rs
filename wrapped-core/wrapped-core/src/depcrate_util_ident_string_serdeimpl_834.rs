// Generated macro for impl_834 (impl)
macro_rules! Depcrate_util_ident_string_serdeimpl_834 {
() => {
// Module: crate::util::ident_string::serde
// Provides: {"impl_834"}
// Dependencies: {}
impl < 'de > serde :: Deserialize < 'de > for IdentString { fn deserialize < D > (deserializer : D) -> std :: result :: Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { deserializer . deserialize_str (IdentStringVisitor) } }
};
}
