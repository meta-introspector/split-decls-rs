// Generated macro for impl_164 (impl)
macro_rules! Depcrate_manifestimpl_164 {
() => {
// Module: crate::manifest
// Provides: {"impl_164"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for InheritableDependency { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { let value = serde_value :: Value :: deserialize (deserializer) ? ; if let Ok (w) = TomlInheritedDependency :: deserialize (serde_value :: ValueDeserializer :: < D :: Error , > :: new (value . clone ())) { return if w . workspace { Ok (InheritableDependency :: Inherit (w)) } else { Err (de :: Error :: custom ("`workspace` cannot be false")) } ; } TomlDependency :: deserialize (serde_value :: ValueDeserializer :: < D :: Error > :: new (value)) . map (InheritableDependency :: Value) } }
};
}
