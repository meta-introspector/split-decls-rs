// Generated macro for DebugBinds (struct)
macro_rules! Depcrate_query_builder_debug_queryDebugBinds {
() => {
// Module: crate::query_builder::debug_query
// Provides: {"DebugBinds"}
// Dependencies: {}
# [doc = " A struct that implements `fmt::Debug` by walking the given AST and writing"] # [doc = " the `fmt::Debug` implementation of each bind parameter."] pub (crate) struct DebugBinds < 'a , DB > { query : & 'a dyn QueryFragment < DB > , }
};
}
