// Generated macro for impl_23 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_23 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "core-net")))] impl Schema for core :: net :: SocketAddr { const SCHEMA : & 'static DataModelType = & DataModelType :: Enum { name : "SocketAddr" , variants : & [& Variant { name : "V4" , data : Data :: Newtype (core :: net :: SocketAddrV4 :: SCHEMA) , } , & Variant { name : "V6" , data : Data :: Newtype (core :: net :: SocketAddrV6 :: SCHEMA) , } ,] , } ; }
};
}
