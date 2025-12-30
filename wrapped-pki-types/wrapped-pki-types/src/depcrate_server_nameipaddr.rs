// Generated macro for IpAddr (enum)
macro_rules! Depcrate_server_nameIpAddr {
() => {
// Module: crate::server_name
// Provides: {"IpAddr"}
// Dependencies: {}
# [doc = " `no_std` implementation of `std::net::IpAddr`."] # [doc = ""] # [doc = " Note: because we intend to replace this type with `core::net::IpAddr` as soon as it is"] # [doc = " stabilized, the identity of this type should not be considered semver-stable. However, the"] # [doc = " attached interfaces are stable; they form a subset of those provided by `core::net::IpAddr`."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub enum IpAddr { # [doc = " An Ipv4 address."] V4 (Ipv4Addr) , # [doc = " An Ipv6 address."] V6 (Ipv6Addr) , }
};
}
