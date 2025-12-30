// Generated macro for impl_509 (impl)
macro_rules! Depcrate_serdeimpl_509 {
() => {
// Module: crate::serde
// Provides: {"impl_509"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Locale { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (ParseVisitor (PhantomData)) } }
};
}
