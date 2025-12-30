// Generated macro for Ipv4Address (struct)
macro_rules! Depcrate_baseIpv4Address {
() => {
// Module: crate::base
// Provides: {"Ipv4Address"}
// Dependencies: {}
# [doc = " IPv4 Address"] # [doc = ""] # [doc = " Binary representation of an IPv4 address. It is encoded in network byte"] # [doc = " order (i.e., big endian). Note that no special alignment restrictions are"] # [doc = " defined by the standard specification."] # [repr (C)] # [derive (Clone , Copy , Debug , Default)] # [derive (Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct Ipv4Address { pub addr : [u8 ; 4] , }
};
}
