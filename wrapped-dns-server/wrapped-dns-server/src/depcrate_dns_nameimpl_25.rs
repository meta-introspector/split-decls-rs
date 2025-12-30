// Generated macro for impl_25 (impl)
macro_rules! Depcrate_dns_nameimpl_25 {
() => {
// Module: crate::dns_name
// Provides: {"impl_25"}
// Dependencies: {}
impl TryFrom < & 'static str > for DnsName { type Error = String ; fn try_from (value : & 'static str) -> Result < Self , Self :: Error > { DnsName :: new (value) } }
};
}
