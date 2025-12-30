// Generated macro for impl_73 (impl)
macro_rules! Depcrate_server_nameimpl_73 {
() => {
// Module: crate::server_name
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for DnsName < 'a > { type Error = InvalidDnsNameError ; fn try_from (value : & 'a str) -> Result < Self , Self :: Error > { DnsName :: try_from_str (value) } }
};
}
