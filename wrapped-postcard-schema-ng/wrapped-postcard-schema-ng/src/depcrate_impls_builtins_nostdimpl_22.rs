// Generated macro for impl_22 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_22 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "core-net")))] impl Schema for core :: net :: SocketAddrV6 { const SCHEMA : & 'static DataModelType = & DataModelType :: Struct { name : "SocketAddrV6" , data : Data :: Struct (& [& NamedField { name : "ip" , ty : core :: net :: Ipv6Addr :: SCHEMA , } , & NamedField { name : "port" , ty : u16 :: SCHEMA , } , & NamedField { name : "flowinfo" , ty : u32 :: SCHEMA , } , & NamedField { name : "scope_id" , ty : u32 :: SCHEMA , } ,]) , } ; }
};
}
