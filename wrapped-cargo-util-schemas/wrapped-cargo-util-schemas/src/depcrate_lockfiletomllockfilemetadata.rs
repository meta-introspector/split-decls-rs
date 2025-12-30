// Generated macro for TomlLockfileMetadata (type)
macro_rules! Depcrate_lockfileTomlLockfileMetadata {
() => {
// Module: crate::lockfile
// Provides: {"TomlLockfileMetadata"}
// Dependencies: {}
# [doc = " Serialization of lockfiles metadata"] # [doc = ""] # [doc = " Older versions of lockfiles have their dependencies' checksums on this `[metadata]` table."] pub type TomlLockfileMetadata = BTreeMap < String , String > ;
};
}
