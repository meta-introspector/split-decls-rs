// Generated macro for impl_20 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_20 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "core-net")))] impl Schema for core :: net :: IpAddr { const SCHEMA : & 'static NamedType = & NamedType { name : "IpAddr" , ty : & DataModelType :: Enum (& [& NamedVariant { name : "V4" , ty : & DataModelVariant :: NewtypeVariant (core :: net :: Ipv4Addr :: SCHEMA) , } , & NamedVariant { name : "V6" , ty : & DataModelVariant :: NewtypeVariant (core :: net :: Ipv6Addr :: SCHEMA) , } ,]) , } ; }
};
}
