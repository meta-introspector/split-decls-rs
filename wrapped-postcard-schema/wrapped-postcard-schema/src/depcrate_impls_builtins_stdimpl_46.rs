// Generated macro for impl_46 (impl)
macro_rules! Depcrate_impls_builtins_stdimpl_46 {
() => {
// Module: crate::impls::builtins_std
// Provides: {"impl_46"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "alloc" , feature = "use-std"))))] impl < K : Schema > Schema for std :: collections :: BTreeSet < K > { const SCHEMA : & 'static NamedType = & NamedType { name : "BTreeSet<K>" , ty : & DataModelType :: Seq (K :: SCHEMA) , } ; }
};
}
