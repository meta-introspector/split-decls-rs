// Generated macro for impl_83 (impl)
macro_rules! Depcrate_lockfileimpl_83 {
() => {
// Module: crate::lockfile
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for TomlLockfileSourceId { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { let s = String :: deserialize (d) ? ; Ok (TomlLockfileSourceId :: new (s) . map_err (de :: Error :: custom) ?) } }
};
}
