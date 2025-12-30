// Generated macro for impl_22 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_22 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "core-net")))] impl Schema for core :: net :: SocketAddrV6 { const SCHEMA : & 'static NamedType = & NamedType { name : "SocketAddrV6" , ty : & DataModelType :: Struct (& [& NamedValue { name : "ip" , ty : core :: net :: Ipv6Addr :: SCHEMA , } , & NamedValue { name : "port" , ty : u16 :: SCHEMA , } , & NamedValue { name : "flowinfo" , ty : u32 :: SCHEMA , } , & NamedValue { name : "scope_id" , ty : u32 :: SCHEMA , } ,]) , } ; }
};
}
