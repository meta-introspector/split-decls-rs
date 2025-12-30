// Generated macro for DisallowedPath (struct)
macro_rules! Depcrate_typesDisallowedPath {
() => {
// Module: crate::types
// Provides: {"DisallowedPath"}
// Dependencies: {}
# [derive (Debug , Serialize)] pub struct DisallowedPath < const REPLACEMENT_ALLOWED : bool = true > { path : String , reason : Option < String > , replacement : Option < String > , # [doc = " Setting `allow_invalid` to true suppresses a warning if `path` does not refer to an existing"] # [doc = " definition."] # [doc = ""] # [doc = " This could be useful when conditional compilation is used, or when a clippy.toml file is"] # [doc = " shared among multiple projects."] allow_invalid : bool , # [doc = " The span of the `DisallowedPath`."] # [doc = ""] # [doc = " Used for diagnostics."] # [serde (skip_serializing)] span : Span , }
};
}
