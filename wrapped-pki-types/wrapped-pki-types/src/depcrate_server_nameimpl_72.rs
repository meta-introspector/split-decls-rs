// Generated macro for impl_72 (impl)
macro_rules! Depcrate_server_nameimpl_72 {
() => {
// Module: crate::server_name
// Provides: {"impl_72"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl TryFrom < String > for DnsName < 'static > { type Error = InvalidDnsNameError ; fn try_from (value : String) -> Result < Self , Self :: Error > { Self :: try_from_string (value) . map_err (| _ | InvalidDnsNameError) } }
};
}
