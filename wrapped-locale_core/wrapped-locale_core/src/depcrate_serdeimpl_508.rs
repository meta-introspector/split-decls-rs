// Generated macro for impl_508 (impl)
macro_rules! Depcrate_serdeimpl_508 {
() => {
// Module: crate::serde
// Provides: {"impl_508"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for LanguageIdentifier { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (ParseVisitor (PhantomData)) } }
};
}
