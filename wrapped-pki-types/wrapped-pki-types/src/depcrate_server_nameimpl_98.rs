// Generated macro for impl_98 (impl)
macro_rules! Depcrate_server_nameimpl_98 {
() => {
// Module: crate::server_name
// Provides: {"impl_98"}
// Dependencies: {}
impl From < [u16 ; 8] > for Ipv6Addr { fn from (value : [u16 ; 8]) -> Self { let addr16 = [value [0] . to_be () , value [1] . to_be () , value [2] . to_be () , value [3] . to_be () , value [4] . to_be () , value [5] . to_be () , value [6] . to_be () , value [7] . to_be () ,] ; Self (unsafe { mem :: transmute :: < [u16 ; 8] , [u8 ; 16] > (addr16) } ,) } }
};
}
