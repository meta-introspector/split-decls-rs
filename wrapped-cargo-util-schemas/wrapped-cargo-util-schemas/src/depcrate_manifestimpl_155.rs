// Generated macro for impl_155 (impl)
macro_rules! Depcrate_manifestimpl_155 {
() => {
// Module: crate::manifest
// Provides: {"impl_155"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for InheritableBtreeMap { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { let value = serde_value :: Value :: deserialize (deserializer) ? ; if let Ok (w) = TomlInheritedField :: deserialize (serde_value :: ValueDeserializer :: < D :: Error > :: new (value . clone ()) ,) { return Ok (InheritableField :: Inherit (w)) ; } BTreeMap :: deserialize (serde_value :: ValueDeserializer :: < D :: Error > :: new (value)) . map (InheritableField :: Value) } }
};
}
