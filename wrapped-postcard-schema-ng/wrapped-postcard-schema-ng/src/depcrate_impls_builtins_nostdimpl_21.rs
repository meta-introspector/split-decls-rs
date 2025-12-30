// Generated macro for impl_21 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_21 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "core-net")))] impl Schema for core :: net :: SocketAddrV4 { const SCHEMA : & 'static DataModelType = & DataModelType :: Struct { name : "SocketAddrV4" , data : Data :: Struct (& [& NamedField { name : "ip" , ty : core :: net :: Ipv4Addr :: SCHEMA , } , & NamedField { name : "port" , ty : u16 :: SCHEMA , } ,]) , } ; }
};
}
