// Generated macro for Contains (trait)
macro_rules! Depcrate_ipnetContains {
() => {
// Module: crate::ipnet
// Provides: {"Contains"}
// Dependencies: {}
# [doc = " Provides a method to test if a network address contains either"] # [doc = " another network address or an IP address."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::net::IpAddr;"] # [doc = " # use ipnet::IpNet;"] # [doc = " #"] # [doc = " let n4_1: IpNet = \"10.1.1.0/24\".parse().unwrap();"] # [doc = " let n4_2: IpNet = \"10.1.1.0/26\".parse().unwrap();"] # [doc = " let n4_3: IpNet = \"10.1.2.0/26\".parse().unwrap();"] # [doc = " let ip4_1: IpAddr = \"10.1.1.1\".parse().unwrap();"] # [doc = " let ip4_2: IpAddr = \"10.1.2.1\".parse().unwrap();"] # [doc = ""] # [doc = " let n6_1: IpNet = \"fd00::/16\".parse().unwrap();"] # [doc = " let n6_2: IpNet = \"fd00::/17\".parse().unwrap();"] # [doc = " let n6_3: IpNet = \"fd01::/17\".parse().unwrap();"] # [doc = " let ip6_1: IpAddr = \"fd00::1\".parse().unwrap();"] # [doc = " let ip6_2: IpAddr = \"fd01::1\".parse().unwrap();"] # [doc = ""] # [doc = " assert!(n4_1.contains(&n4_2));"] # [doc = " assert!(!n4_1.contains(&n4_3));"] # [doc = " assert!(n4_1.contains(&ip4_1));"] # [doc = " assert!(!n4_1.contains(&ip4_2));"] # [doc = ""] # [doc = " assert!(n6_1.contains(&n6_2));"] # [doc = " assert!(!n6_1.contains(&n6_3));"] # [doc = " assert!(n6_1.contains(&ip6_1));"] # [doc = " assert!(!n6_1.contains(&ip6_2));"] # [doc = ""] # [doc = " assert!(!n4_1.contains(&n6_1) && !n6_1.contains(&n4_1));"] # [doc = " assert!(!n4_1.contains(&ip6_1) && !n6_1.contains(&ip4_1));"] # [doc = " ```"] pub trait Contains < T > { fn contains (& self , other : T) -> bool ; }
};
}
