// Generated macro for impl_63 (impl)
macro_rules! Depcrate_serde_implimpl_63 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for GlobSet { fn deserialize < D : Deserializer < 'de > > (deserializer : D ,) -> Result < Self , D :: Error > { deserializer . deserialize_seq (GlobSetVisitor) } }
};
}
