// Generated macro for Ipv6Address (struct)
macro_rules! Depcrate_baseIpv6Address {
() => {
// Module: crate::base
// Provides: {"Ipv6Address"}
// Dependencies: {}
# [doc = " IPv6 Address"] # [doc = ""] # [doc = " Binary representation of an IPv6 address, encoded in network byte order"] # [doc = " (i.e., big endian). Similar to the IPv4 address, no special alignment"] # [doc = " restrictions are defined by the standard specification."] # [repr (C)] # [derive (Clone , Copy , Debug)] # [derive (Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct Ipv6Address { pub addr : [u8 ; 16] , }
};
}
