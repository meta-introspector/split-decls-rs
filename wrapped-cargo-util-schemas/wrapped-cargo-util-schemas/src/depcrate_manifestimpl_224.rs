// Generated macro for impl_224 (impl)
macro_rules! Depcrate_manifestimpl_224 {
() => {
// Module: crate::manifest
// Provides: {"impl_224"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for TomlLint { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . string (| string | { TomlLintLevel :: deserialize (string . into_deserializer ()) . map (TomlLint :: Level) }) . map (| map | map . deserialize () . map (TomlLint :: Config)) . deserialize (deserializer) } }
};
}
