// Generated macro for IpBitOr (trait)
macro_rules! Depcrate_ipextIpBitOr {
() => {
// Module: crate::ipext
// Provides: {"IpBitOr"}
// Dependencies: {}
# [doc = " Provides a `bitor()` method for `Ipv4Addr` and `Ipv6Addr`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(not(feature = \"std\"))]"] # [doc = " # use core::net::{Ipv4Addr, Ipv6Addr};"] # [doc = " # #[cfg(feature = \"std\")]"] # [doc = " use std::net::{Ipv4Addr, Ipv6Addr};"] # [doc = " use ipnet::IpBitOr;"] # [doc = ""] # [doc = " let ip: Ipv4Addr = \"10.1.1.1\".parse().unwrap();"] # [doc = " let mask: Ipv4Addr = \"0.0.0.255\".parse().unwrap();"] # [doc = " let res: Ipv4Addr = \"10.1.1.255\".parse().unwrap();"] # [doc = ""] # [doc = " assert_eq!(ip.bitor(mask), res);"] # [doc = " assert_eq!(ip.bitor(0x000000ff), res);"] # [doc = " "] # [doc = " let ip: Ipv6Addr = \"fd00::1\".parse().unwrap();"] # [doc = " let mask: Ipv6Addr = \"::ffff:ffff\".parse().unwrap();"] # [doc = " let res: Ipv6Addr = \"fd00::ffff:ffff\".parse().unwrap();"] # [doc = ""] # [doc = " assert_eq!(ip.bitor(mask), res);"] # [doc = " assert_eq!(ip.bitor(u128::from(0xffffffffu32)), res);"] # [doc = " ```"] pub trait IpBitOr < RHS = Self > { type Output ; fn bitor (self , rhs : RHS) -> Self :: Output ; }
};
}
