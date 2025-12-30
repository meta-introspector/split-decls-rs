// Generated macro for ip_addr_std_derive_impl (module)
macro_rules! Depcrate_schemaip_addr_std_derive_impl {
() => {
// Module: crate::schema
// Provides: {"ip_addr_std_derive_impl"}
// Dependencies: {}
# [cfg (feature = "std")] mod ip_addr_std_derive_impl { use crate :: BorshSchema as BorshSchemaMacro ; # [derive (BorshSchemaMacro)] # [borsh (crate = "crate")] pub struct Ipv4Addr { octets : [u8 ; 4] , } # [derive (BorshSchemaMacro)] # [borsh (crate = "crate")] pub struct Ipv6Addr { octets : [u8 ; 16] , } # [derive (BorshSchemaMacro)] # [borsh (crate = "crate")] pub enum IpAddr { # [doc = " An IPv4 address."] V4 (std :: net :: Ipv4Addr) , # [doc = " An IPv6 address."] V6 (std :: net :: Ipv6Addr) , } }
};
}
