// Generated macro for SourceKind (enum)
macro_rules! Depcrate_core_source_kindSourceKind {
() => {
// Module: crate::core::source_kind
// Provides: {"SourceKind"}
// Dependencies: {}
# [doc = " The possible kinds of code source."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum SourceKind { # [doc = " A git repository."] Git (GitReference) , # [doc = " A local path."] Path , # [doc = " A remote registry."] Registry , # [doc = " A sparse registry."] SparseRegistry , # [doc = " A local filesystem-based registry."] LocalRegistry , # [doc = " A directory-based registry."] Directory , }
};
}
