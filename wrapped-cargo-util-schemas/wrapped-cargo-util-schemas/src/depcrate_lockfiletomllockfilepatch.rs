// Generated macro for TomlLockfilePatch (struct)
macro_rules! Depcrate_lockfileTomlLockfilePatch {
() => {
// Module: crate::lockfile
// Provides: {"TomlLockfilePatch"}
// Dependencies: {}
# [doc = " Serialization of unused patches"] # [doc = ""] # [doc = " Cargo stores patches that were declared but not used during resolution."] # [derive (Serialize , Deserialize , Debug , Default)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlLockfilePatch { # [doc = " The list of unused dependency patches."] pub unused : Vec < TomlLockfileDependency > , }
};
}
