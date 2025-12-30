// Generated macro for impl_20 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_20 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "core-net")))] impl Schema for core :: net :: IpAddr { const SCHEMA : & 'static DataModelType = & DataModelType :: Enum { name : "IpAddr" , variants : & [& Variant { name : "V4" , data : Data :: Newtype (core :: net :: Ipv4Addr :: SCHEMA) , } , & Variant { name : "V6" , data : Data :: Newtype (core :: net :: Ipv6Addr :: SCHEMA) , } ,] , } ; }
};
}
