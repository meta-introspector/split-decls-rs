// Generated macro for impl_35 (impl)
macro_rules! Depcrate_impls_builtins_allocimpl_35 {
() => {
// Module: crate::impls::builtins_alloc
// Provides: {"impl_35"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "alloc" , feature = "use-std"))))] impl < K : Schema > Schema for alloc :: collections :: BTreeSet < K > { const SCHEMA : & 'static DataModelType = & DataModelType :: Seq (K :: SCHEMA) ; }
};
}
