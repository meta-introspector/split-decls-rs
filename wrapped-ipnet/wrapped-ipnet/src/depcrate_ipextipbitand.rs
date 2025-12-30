// Generated macro for IpBitAnd (trait)
macro_rules! Depcrate_ipextIpBitAnd {
() => {
// Module: crate::ipext
// Provides: {"IpBitAnd"}
// Dependencies: {}
# [doc = " Provides a `bitand()` method for `Ipv4Addr` and `Ipv6Addr`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(not(feature = \"std\"))]"] # [doc = " # use core::net::{Ipv4Addr, Ipv6Addr};"] # [doc = " # #[cfg(feature = \"std\")]"] # [doc = " use std::net::{Ipv4Addr, Ipv6Addr};"] # [doc = " use ipnet::IpBitAnd;"] # [doc = ""] # [doc = " let ip: Ipv4Addr = \"192.168.1.1\".parse().unwrap();"] # [doc = " let mask: Ipv4Addr = \"255.255.0.0\".parse().unwrap();"] # [doc = " let res: Ipv4Addr = \"192.168.0.0\".parse().unwrap();"] # [doc = ""] # [doc = " assert_eq!(ip.bitand(mask), res);"] # [doc = " assert_eq!(ip.bitand(0xffff0000), res);"] # [doc = " "] # [doc = " let ip: Ipv6Addr = \"fd00:1234::1\".parse().unwrap();"] # [doc = " let mask: Ipv6Addr = \"ffff::\".parse().unwrap();"] # [doc = " let res: Ipv6Addr = \"fd00::\".parse().unwrap();"] # [doc = ""] # [doc = " assert_eq!(ip.bitand(mask), res);"] # [doc = " assert_eq!(ip.bitand(0xffff_0000_0000_0000_0000_0000_0000_0000u128), res);"] # [doc = " ```"] pub trait IpBitAnd < RHS = Self > { type Output ; fn bitand (self , rhs : RHS) -> Self :: Output ; }
};
}
