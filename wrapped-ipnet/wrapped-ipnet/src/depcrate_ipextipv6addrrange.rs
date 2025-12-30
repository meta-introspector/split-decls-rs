// Generated macro for Ipv6AddrRange (struct)
macro_rules! Depcrate_ipextIpv6AddrRange {
() => {
// Module: crate::ipext
// Provides: {"Ipv6AddrRange"}
// Dependencies: {}
# [doc = " An `Iterator` over a range of IPv6 addresses."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ``` "] # [doc = " # #[cfg(not(feature = \"std\"))]"] # [doc = " # use core::net::Ipv6Addr;"] # [doc = " # #[cfg(feature = \"std\")]"] # [doc = " use std::net::Ipv6Addr;"] # [doc = " use ipnet::Ipv6AddrRange;"] # [doc = ""] # [doc = " let hosts = Ipv6AddrRange::new("] # [doc = "     \"fd00::\".parse().unwrap(),"] # [doc = "     \"fd00::3\".parse().unwrap(),"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!(hosts.collect::<Vec<Ipv6Addr>>(), vec!["] # [doc = "     \"fd00::\".parse::<Ipv6Addr>().unwrap(),"] # [doc = "     \"fd00::1\".parse().unwrap(),"] # [doc = "     \"fd00::2\".parse().unwrap(),"] # [doc = "     \"fd00::3\".parse().unwrap(),"] # [doc = " ]);"] # [doc = " ```"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct Ipv6AddrRange { start : Ipv6Addr , end : Ipv6Addr , }
};
}
