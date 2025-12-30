// Generated macro for impl_97 (impl)
macro_rules! Depcrate_server_nameimpl_97 {
() => {
// Module: crate::server_name
// Provides: {"impl_97"}
// Dependencies: {}
impl TryFrom < & str > for Ipv6Addr { type Error = AddrParseError ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Parser :: new (value . as_bytes ()) . parse_with (| p | p . read_ipv6_addr () , AddrKind :: Ipv6) } }
};
}
