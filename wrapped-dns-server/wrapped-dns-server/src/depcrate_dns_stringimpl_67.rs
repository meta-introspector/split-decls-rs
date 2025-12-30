// Generated macro for impl_67 (impl)
macro_rules! Depcrate_dns_stringimpl_67 {
() => {
// Module: crate::dns_string
// Provides: {"impl_67"}
// Dependencies: {}
impl TryFrom < & 'static str > for DnsString { type Error = String ; fn try_from (value : & 'static str) -> Result < Self , Self :: Error > { DnsString :: new (value) } }
};
}
