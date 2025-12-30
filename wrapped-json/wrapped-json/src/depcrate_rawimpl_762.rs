// Generated macro for impl_762 (impl)
macro_rules! Depcrate_rawimpl_762 {
() => {
// Module: crate::raw
// Provides: {"impl_762"}
// Dependencies: {}
impl < 'de > DeserializeSeed < 'de > for BoxedFromString { type Value = Box < RawValue > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (self) } }
};
}
