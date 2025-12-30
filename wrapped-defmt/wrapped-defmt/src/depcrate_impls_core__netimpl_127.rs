// Generated macro for impl_127 (impl)
macro_rules! Depcrate_impls_core__netimpl_127 {
() => {
// Module: crate::impls::core_::net
// Provides: {"impl_127"}
// Dependencies: {}
impl Format for net :: Ipv6Addr { fn format (& self , fmt : Formatter) { let octets : [u8 ; 16] = self . octets () ; crate :: write ! (fmt , "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}" , octets [0] , octets [1] , octets [2] , octets [3] , octets [4] , octets [5] , octets [6] , octets [7] , octets [8] , octets [9] , octets [10] , octets [11] , octets [12] , octets [13] , octets [14] , octets [15]) ; } }
};
}
