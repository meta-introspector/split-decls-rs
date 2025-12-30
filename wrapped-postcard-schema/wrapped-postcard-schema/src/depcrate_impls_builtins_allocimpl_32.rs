// Generated macro for impl_32 (impl)
macro_rules! Depcrate_impls_builtins_allocimpl_32 {
() => {
// Module: crate::impls::builtins_alloc
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "alloc" , feature = "use-std"))))] impl < T : Schema > Schema for alloc :: vec :: Vec < T > { const SCHEMA : & 'static NamedType = & NamedType { name : "Vec<T>" , ty : & DataModelType :: Seq (T :: SCHEMA) , } ; }
};
}
