// Generated macro for IpNet (enum)
macro_rules! Depcrate_ipnetIpNet {
() => {
// Module: crate::ipnet
// Provides: {"IpNet"}
// Dependencies: {}
# [doc = " An IP network address, either IPv4 or IPv6."] # [doc = ""] # [doc = " This enum can contain either an [`Ipv4Net`] or an [`Ipv6Net`]. A"] # [doc = " [`From`] implementation is provided to convert these into an"] # [doc = " `IpNet`."] # [doc = ""] # [doc = " # Textual representation"] # [doc = ""] # [doc = " `IpNet` provides a [`FromStr`] implementation for parsing network"] # [doc = " addresses represented in CIDR notation. See [IETF RFC 4632] for the"] # [doc = " CIDR notation."] # [doc = ""] # [doc = " [`Ipv4Net`]: struct.Ipv4Net.html"] # [doc = " [`Ipv6Net`]: struct.Ipv6Net.html"] # [doc = " [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html"] # [doc = " [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html"] # [doc = " [IETF RFC 4632]: https://tools.ietf.org/html/rfc4632"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::net::IpAddr;"] # [doc = " use ipnet::IpNet;"] # [doc = ""] # [doc = " let net: IpNet = \"10.1.1.0/24\".parse().unwrap();"] # [doc = " assert_eq!(Ok(net.network()), \"10.1.1.0\".parse());"] # [doc = ""] # [doc = " let net: IpNet = \"fd00::/32\".parse().unwrap();"] # [doc = " assert_eq!(Ok(net.network()), \"fd00::\".parse());"] # [doc = " ```"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum IpNet { V4 (Ipv4Net) , V6 (Ipv6Net) , }
};
}
