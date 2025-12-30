// Generated macro for impl_45 (impl)
macro_rules! Depcrate_core_source_kindimpl_45 {
() => {
// Module: crate::core::source_kind
// Provides: {"impl_45"}
// Dependencies: {}
impl SourceKind { pub fn protocol (& self) -> Option < & str > { match self { SourceKind :: Path => Some ("path") , SourceKind :: Git (_) => Some ("git") , SourceKind :: Registry => Some ("registry") , SourceKind :: SparseRegistry => None , SourceKind :: LocalRegistry => Some ("local-registry") , SourceKind :: Directory => Some ("directory") , } } }
};
}
