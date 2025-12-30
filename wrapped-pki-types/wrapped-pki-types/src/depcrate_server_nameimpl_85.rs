// Generated macro for impl_85 (impl)
macro_rules! Depcrate_server_nameimpl_85 {
() => {
// Module: crate::server_name
// Provides: {"impl_85"}
// Dependencies: {}
impl TryFrom < & str > for IpAddr { type Error = AddrParseError ; fn try_from (value : & str) -> Result < Self , Self :: Error > { match Ipv4Addr :: try_from (value) { Ok (v4) => Ok (Self :: V4 (v4)) , Err (_) => match Ipv6Addr :: try_from (value) { Ok (v6) => Ok (Self :: V6 (v6)) , Err (e) => Err (e) , } , } } }
};
}
