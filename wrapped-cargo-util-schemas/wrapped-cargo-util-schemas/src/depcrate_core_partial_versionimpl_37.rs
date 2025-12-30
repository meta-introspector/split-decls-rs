// Generated macro for impl_37 (impl)
macro_rules! Depcrate_core_partial_versionimpl_37 {
() => {
// Module: crate::core::partial_version
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'de > serde :: Deserialize < 'de > for PartialVersion { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . expecting ("SemVer version") . string (| value | value . parse () . map_err (serde :: de :: Error :: custom)) . deserialize (deserializer) } }
};
}
