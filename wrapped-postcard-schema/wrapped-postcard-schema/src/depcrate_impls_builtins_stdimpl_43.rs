// Generated macro for impl_43 (impl)
macro_rules! Depcrate_impls_builtins_stdimpl_43 {
() => {
// Module: crate::impls::builtins_std
// Provides: {"impl_43"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "alloc" , feature = "use-std"))))] impl < K : Schema , V : Schema > Schema for std :: collections :: HashMap < K , V > { const SCHEMA : & 'static NamedType = & NamedType { name : "HashMap<K, V>" , ty : & DataModelType :: Map { key : K :: SCHEMA , val : V :: SCHEMA , } , } ; }
};
}
