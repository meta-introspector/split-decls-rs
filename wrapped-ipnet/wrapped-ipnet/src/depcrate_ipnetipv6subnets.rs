// Generated macro for Ipv6Subnets (struct)
macro_rules! Depcrate_ipnetIpv6Subnets {
() => {
// Module: crate::ipnet
// Provides: {"Ipv6Subnets"}
// Dependencies: {}
# [doc = " An `Iterator` that generates IPv6 network addresses."] # [doc = ""] # [doc = " Generates the subnets between the provided `start` and `end` IP"] # [doc = " addresses inclusive of `end`. Each iteration generates the next"] # [doc = " network address of the largest valid size it can, while using a"] # [doc = " prefix length not less than `min_prefix_len`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::net::Ipv6Addr;"] # [doc = " # use std::str::FromStr;"] # [doc = " # use ipnet::{Ipv6Net, Ipv6Subnets};"] # [doc = " let subnets = Ipv6Subnets::new("] # [doc = "     \"fd00::\".parse().unwrap(),"] # [doc = "     \"fd00:ef:ffff:ffff:ffff:ffff:ffff:ffff\".parse().unwrap(),"] # [doc = "     26,"] # [doc = " );"] # [doc = " "] # [doc = " assert_eq!(subnets.collect::<Vec<Ipv6Net>>(), vec!["] # [doc = "     \"fd00::/26\".parse().unwrap(),"] # [doc = "     \"fd00:40::/26\".parse().unwrap(),"] # [doc = "     \"fd00:80::/26\".parse().unwrap(),"] # [doc = "     \"fd00:c0::/27\".parse().unwrap(),"] # [doc = "     \"fd00:e0::/28\".parse().unwrap(),"] # [doc = " ]);"] # [doc = " ```"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct Ipv6Subnets { start : Ipv6Addr , end : Ipv6Addr , min_prefix_len : u8 , }
};
}
