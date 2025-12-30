// Generated macro for impl_125 (impl)
macro_rules! Depcrate_manifest_rust_versionimpl_125 {
() => {
// Module: crate::manifest::rust_version
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'de > serde :: Deserialize < 'de > for RustVersion { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . expecting ("SemVer version") . string (| value | value . parse () . map_err (serde :: de :: Error :: custom)) . deserialize (deserializer) } }
};
}
