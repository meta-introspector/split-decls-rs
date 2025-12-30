// Generated macro for impl_60 (impl)
macro_rules! Depcrate_serde_implimpl_60 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Glob { fn deserialize < D : Deserializer < 'de > > (deserializer : D ,) -> Result < Self , D :: Error > { deserializer . deserialize_str (GlobVisitor) } }
};
}
