// Generated macro for Ipv6Net (struct)
macro_rules! Depcrate_ipnetIpv6Net {
() => {
// Module: crate::ipnet
// Provides: {"Ipv6Net"}
// Dependencies: {}
# [doc = " An IPv6 network address."] # [doc = ""] # [doc = " See [`IpNet`] for a type encompassing both IPv4 and IPv6 network"] # [doc = " addresses."] # [doc = ""] # [doc = " # Textual representation"] # [doc = ""] # [doc = " `Ipv6Net` provides a [`FromStr`] implementation for parsing network"] # [doc = " addresses represented in CIDR notation. See [IETF RFC 4632] for the"] # [doc = " CIDR notation."] # [doc = ""] # [doc = " [`IpNet`]: enum.IpNet.html"] # [doc = " [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html"] # [doc = " [IETF RFC 4632]: https://tools.ietf.org/html/rfc4632"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::net::Ipv6Addr;"] # [doc = " use ipnet::Ipv6Net;"] # [doc = ""] # [doc = " let net: Ipv6Net = \"fd00::/32\".parse().unwrap();"] # [doc = " assert_eq!(Ok(net.network()), \"fd00::\".parse());"] # [doc = " ```"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct Ipv6Net { addr : Ipv6Addr , prefix_len : u8 , }
};
}
