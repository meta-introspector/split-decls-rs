// Generated macro for impl_34 (impl)
macro_rules! Depcrate_impls_builtins_allocimpl_34 {
() => {
// Module: crate::impls::builtins_alloc
// Provides: {"impl_34"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "alloc" , feature = "use-std"))))] impl < K : Schema , V : Schema > Schema for alloc :: collections :: BTreeMap < K , V > { const SCHEMA : & 'static DataModelType = & DataModelType :: Map { key : K :: SCHEMA , val : V :: SCHEMA , } ; }
};
}
