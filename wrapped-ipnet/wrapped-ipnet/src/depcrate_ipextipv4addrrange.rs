// Generated macro for Ipv4AddrRange (struct)
macro_rules! Depcrate_ipextIpv4AddrRange {
() => {
// Module: crate::ipext
// Provides: {"Ipv4AddrRange"}
// Dependencies: {}
# [doc = " An `Iterator` over a range of IPv4 addresses."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(not(feature = \"std\"))]"] # [doc = " # use core::net::Ipv4Addr;"] # [doc = " # #[cfg(feature = \"std\")]"] # [doc = " use std::net::Ipv4Addr;"] # [doc = " use ipnet::Ipv4AddrRange;"] # [doc = ""] # [doc = " let hosts = Ipv4AddrRange::new("] # [doc = "     \"10.0.0.0\".parse().unwrap(),"] # [doc = "     \"10.0.0.3\".parse().unwrap(),"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!(hosts.collect::<Vec<Ipv4Addr>>(), vec!["] # [doc = "     \"10.0.0.0\".parse::<Ipv4Addr>().unwrap(),"] # [doc = "     \"10.0.0.1\".parse().unwrap(),"] # [doc = "     \"10.0.0.2\".parse().unwrap(),"] # [doc = "     \"10.0.0.3\".parse().unwrap(),"] # [doc = " ]);"] # [doc = " ```"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct Ipv4AddrRange { start : Ipv4Addr , end : Ipv4Addr , }
};
}
