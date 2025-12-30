// Generated macro for impl_74 (impl)
macro_rules! Depcrate_server_nameimpl_74 {
() => {
// Module: crate::server_name
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for DnsName < 'a > { type Error = InvalidDnsNameError ; fn try_from (value : & 'a [u8]) -> Result < Self , Self :: Error > { validate (value) ? ; Ok (Self (DnsNameInner :: Borrowed (str :: from_utf8 (value) . unwrap ()))) } }
};
}
