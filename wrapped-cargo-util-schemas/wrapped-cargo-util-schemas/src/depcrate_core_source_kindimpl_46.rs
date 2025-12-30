// Generated macro for impl_46 (impl)
macro_rules! Depcrate_core_source_kindimpl_46 {
() => {
// Module: crate::core::source_kind
// Provides: {"impl_46"}
// Dependencies: {}
impl Ord for SourceKind { fn cmp (& self , other : & SourceKind) -> Ordering { match (self , other) { (SourceKind :: Path , SourceKind :: Path) => Ordering :: Equal , (SourceKind :: Path , _) => Ordering :: Less , (_ , SourceKind :: Path) => Ordering :: Greater , (SourceKind :: Registry , SourceKind :: Registry) => Ordering :: Equal , (SourceKind :: Registry , _) => Ordering :: Less , (_ , SourceKind :: Registry) => Ordering :: Greater , (SourceKind :: SparseRegistry , SourceKind :: SparseRegistry) => Ordering :: Equal , (SourceKind :: SparseRegistry , _) => Ordering :: Less , (_ , SourceKind :: SparseRegistry) => Ordering :: Greater , (SourceKind :: LocalRegistry , SourceKind :: LocalRegistry) => Ordering :: Equal , (SourceKind :: LocalRegistry , _) => Ordering :: Less , (_ , SourceKind :: LocalRegistry) => Ordering :: Greater , (SourceKind :: Directory , SourceKind :: Directory) => Ordering :: Equal , (SourceKind :: Directory , _) => Ordering :: Less , (_ , SourceKind :: Directory) => Ordering :: Greater , (SourceKind :: Git (a) , SourceKind :: Git (b)) => a . cmp (b) , } } }
};
}
