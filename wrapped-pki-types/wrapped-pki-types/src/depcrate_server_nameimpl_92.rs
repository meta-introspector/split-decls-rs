// Generated macro for impl_92 (impl)
macro_rules! Depcrate_server_nameimpl_92 {
() => {
// Module: crate::server_name
// Provides: {"impl_92"}
// Dependencies: {}
impl TryFrom < & str > for Ipv4Addr { type Error = AddrParseError ; fn try_from (value : & str) -> Result < Self , Self :: Error > { if value . len () > 15 { Err (AddrParseError (AddrKind :: Ipv4)) } else { Parser :: new (value . as_bytes ()) . parse_with (| p | p . read_ipv4_addr () , AddrKind :: Ipv4) } } }
};
}
