// Generated macro for impl_93 (impl)
macro_rules! Depcrate_lockfileimpl_93 {
() => {
// Module: crate::lockfile
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for TomlLockfilePackageId { fn deserialize < D > (d : D) -> Result < TomlLockfilePackageId , D :: Error > where D : de :: Deserializer < 'de > , { String :: deserialize (d) . and_then (| string | { string . parse :: < TomlLockfilePackageId > () . map_err (de :: Error :: custom) }) } }
};
}
