// Generated macro for impl_44 (impl)
macro_rules! Depcrate_core_source_kindimpl_44 {
() => {
// Module: crate::core::source_kind
// Provides: {"impl_44"}
// Dependencies: {}
impl std :: hash :: Hash for SourceKind { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { core :: mem :: discriminant (self) . hash (state) ; if let SourceKind :: Git (git) = self { git . hash (state) ; } } }
};
}
