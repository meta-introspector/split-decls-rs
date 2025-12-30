// Generated macro for impl_36 (impl)
macro_rules! Depcrate_impls_builtins_allocimpl_36 {
() => {
// Module: crate::impls::builtins_alloc
// Provides: {"impl_36"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "alloc" , feature = "use-std"))))] impl < T : Schema > Schema for alloc :: boxed :: Box < T > { const SCHEMA : & 'static NamedType = & NamedType { name : "Box<T>" , ty : T :: SCHEMA . ty , } ; }
};
}
