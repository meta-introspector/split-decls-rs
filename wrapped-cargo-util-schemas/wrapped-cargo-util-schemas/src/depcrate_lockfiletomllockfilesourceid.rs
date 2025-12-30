// Generated macro for TomlLockfileSourceId (struct)
macro_rules! Depcrate_lockfileTomlLockfileSourceId {
() => {
// Module: crate::lockfile
// Provides: {"TomlLockfileSourceId"}
// Dependencies: {}
# [doc = " Serialization of dependency's source"] # [derive (Debug , Clone)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema) , schemars (with = "String"))] pub struct TomlLockfileSourceId { # [doc = " The string representation of the source as it appears in the lockfile."] source_str : String , # [doc = " The parsed source type, e.g. `git`, `registry`."] # [doc = ""] # [doc = " Used for sources ordering."] kind : SourceKind , # [doc = " The parsed URL of the source."] # [doc = ""] # [doc = " Used for sources ordering."] url : Url , }
};
}
