// Generated macro for Ipv4Net (struct)
macro_rules! Depcrate_ipnetIpv4Net {
() => {
// Module: crate::ipnet
// Provides: {"Ipv4Net"}
// Dependencies: {}
# [doc = " An IPv4 network address."] # [doc = ""] # [doc = " See [`IpNet`] for a type encompassing both IPv4 and IPv6 network"] # [doc = " addresses."] # [doc = ""] # [doc = " # Textual representation"] # [doc = ""] # [doc = " `Ipv4Net` provides a [`FromStr`] implementation for parsing network"] # [doc = " addresses represented in CIDR notation. See [IETF RFC 4632] for the"] # [doc = " CIDR notation."] # [doc = ""] # [doc = " [`IpNet`]: enum.IpNet.html"] # [doc = " [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html"] # [doc = " [IETF RFC 4632]: https://tools.ietf.org/html/rfc4632"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"std\")]"] # [doc = " # use std::net::Ipv6Addr;"] # [doc = " # #[cfg(not(feature = \"std\"))]"] # [doc = " # use core::net::Ipv6Addr;"] # [doc = " use ipnet::Ipv4Net;"] # [doc = ""] # [doc = " let net: Ipv4Net = \"10.1.1.0/24\".parse().unwrap();"] # [doc = " assert_eq!(Ok(net.network()), \"10.1.1.0\".parse());"] # [doc = " ```"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct Ipv4Net { addr : Ipv4Addr , prefix_len : u8 , }
};
}
