// Generated macro for impl_23 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_23 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "core-net")))] impl Schema for core :: net :: SocketAddr { const SCHEMA : & 'static NamedType = & NamedType { name : "SocketAddr" , ty : & DataModelType :: Enum (& [& NamedVariant { name : "V4" , ty : & DataModelVariant :: NewtypeVariant (core :: net :: SocketAddrV4 :: SCHEMA) , } , & NamedVariant { name : "V6" , ty : & DataModelVariant :: NewtypeVariant (core :: net :: SocketAddrV6 :: SCHEMA) , } ,]) , } ; }
};
}
