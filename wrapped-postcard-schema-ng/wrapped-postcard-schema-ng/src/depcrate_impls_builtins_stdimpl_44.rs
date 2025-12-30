// Generated macro for impl_44 (impl)
macro_rules! Depcrate_impls_builtins_stdimpl_44 {
() => {
// Module: crate::impls::builtins_std
// Provides: {"impl_44"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "alloc" , feature = "use-std"))))] impl < K : Schema , V : Schema > Schema for std :: collections :: BTreeMap < K , V > { const SCHEMA : & 'static DataModelType = & DataModelType :: Map { key : K :: SCHEMA , val : V :: SCHEMA , } ; }
};
}
