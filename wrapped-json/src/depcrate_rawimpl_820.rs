// Generated macro for impl_820 (impl)
macro_rules! Depcrate_rawimpl_820 {
() => {
// Module: crate::raw
// Provides: {"impl_820"}
// Dependencies: {}
impl < 'de > DeserializeSeed < 'de > for ReferenceFromString { type Value = & 'de RawValue ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (self) } }
};
}
