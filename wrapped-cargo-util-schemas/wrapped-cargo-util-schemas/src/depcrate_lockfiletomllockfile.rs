// Generated macro for TomlLockfile (struct)
macro_rules! Depcrate_lockfileTomlLockfile {
() => {
// Module: crate::lockfile
// Provides: {"TomlLockfile"}
// Dependencies: {}
# [doc = " Serialization of `Cargo.lock`"] # [derive (Serialize , Deserialize , Debug)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlLockfile { # [doc = " The lockfile format version (`version =` field)."] # [doc = ""] # [doc = " This field is optional for backward compatibility. Older lockfiles, i.e. V1 and V2, does"] # [doc = " not have the version field serialized."] pub version : Option < u32 > , # [doc = " The list of `[[package]]` entries describing each resolved dependency."] pub package : Option < Vec < TomlLockfileDependency > > , # [doc = " The `[root]` table describing the root package."] # [doc = ""] # [doc = " This field is optional for backward compatibility. Older lockfiles have the root package"] # [doc = " separated, whereas newer lockfiles have the root package as part of `[[package]]`."] pub root : Option < TomlLockfileDependency > , # [doc = " The `[metadata]` table"] # [doc = ""] # [doc = ""] # [doc = " In older lockfile versions, dependency checksums were stored here instead of alongside each"] # [doc = " package entry."] pub metadata : Option < TomlLockfileMetadata > , # [doc = " The `[patch]` table describing unused patches."] # [doc = ""] # [doc = " The lockfile stores them as a list of `[[patch.unused]]` entries."] # [serde (default , skip_serializing_if = "TomlLockfilePatch::is_empty")] pub patch : TomlLockfilePatch , }
};
}
