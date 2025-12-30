// Generated macro for Ipv4Subnets (struct)
macro_rules! Depcrate_ipnetIpv4Subnets {
() => {
// Module: crate::ipnet
// Provides: {"Ipv4Subnets"}
// Dependencies: {}
# [doc = " An `Iterator` that generates IPv4 network addresses."] # [doc = ""] # [doc = " Generates the subnets between the provided `start` and `end` IP"] # [doc = " addresses inclusive of `end`. Each iteration generates the next"] # [doc = " network address of the largest valid size it can, while using a"] # [doc = " prefix length not less than `min_prefix_len`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::net::Ipv4Addr;"] # [doc = " # use std::str::FromStr;"] # [doc = " # use ipnet::{Ipv4Net, Ipv4Subnets};"] # [doc = " let subnets = Ipv4Subnets::new("] # [doc = "     \"10.0.0.0\".parse().unwrap(),"] # [doc = "     \"10.0.0.239\".parse().unwrap(),"] # [doc = "     26,"] # [doc = " );"] # [doc = " "] # [doc = " assert_eq!(subnets.collect::<Vec<Ipv4Net>>(), vec!["] # [doc = "     \"10.0.0.0/26\".parse().unwrap(),"] # [doc = "     \"10.0.0.64/26\".parse().unwrap(),"] # [doc = "     \"10.0.0.128/26\".parse().unwrap(),"] # [doc = "     \"10.0.0.192/27\".parse().unwrap(),"] # [doc = "     \"10.0.0.224/28\".parse().unwrap(),"] # [doc = " ]);"] # [doc = " ```"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct Ipv4Subnets { start : Ipv4Addr , end : Ipv4Addr , min_prefix_len : u8 , }
};
}
