// Generated macro for IpAddrRange (enum)
macro_rules! Depcrate_ipextIpAddrRange {
() => {
// Module: crate::ipext
// Provides: {"IpAddrRange"}
// Dependencies: {}
# [doc = " An `Iterator` over a range of IP addresses, either IPv4 or IPv6."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::net::IpAddr;"] # [doc = " use ipnet::{IpAddrRange, Ipv4AddrRange, Ipv6AddrRange};"] # [doc = ""] # [doc = " let hosts = IpAddrRange::from(Ipv4AddrRange::new("] # [doc = "     \"10.0.0.0\".parse().unwrap(),"] # [doc = "     \"10.0.0.3\".parse().unwrap(),"] # [doc = " ));"] # [doc = ""] # [doc = " assert_eq!(hosts.collect::<Vec<IpAddr>>(), vec!["] # [doc = "     \"10.0.0.0\".parse::<IpAddr>().unwrap(),"] # [doc = "     \"10.0.0.1\".parse().unwrap(),"] # [doc = "     \"10.0.0.2\".parse().unwrap(),"] # [doc = "     \"10.0.0.3\".parse().unwrap(),"] # [doc = " ]);"] # [doc = ""] # [doc = " let hosts = IpAddrRange::from(Ipv6AddrRange::new("] # [doc = "     \"fd00::\".parse().unwrap(),"] # [doc = "     \"fd00::3\".parse().unwrap(),"] # [doc = " ));"] # [doc = ""] # [doc = " assert_eq!(hosts.collect::<Vec<IpAddr>>(), vec!["] # [doc = "     \"fd00::0\".parse::<IpAddr>().unwrap(),"] # [doc = "     \"fd00::1\".parse().unwrap(),"] # [doc = "     \"fd00::2\".parse().unwrap(),"] # [doc = "     \"fd00::3\".parse().unwrap(),"] # [doc = " ]);"] # [doc = " ```"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub enum IpAddrRange { V4 (Ipv4AddrRange) , V6 (Ipv6AddrRange) , }
};
}
