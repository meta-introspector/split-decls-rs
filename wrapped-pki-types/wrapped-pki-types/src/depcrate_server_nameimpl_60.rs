// Generated macro for impl_60 (impl)
macro_rules! Depcrate_server_nameimpl_60 {
() => {
// Module: crate::server_name
// Provides: {"impl_60"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl TryFrom < String > for ServerName < 'static > { type Error = InvalidDnsNameError ; fn try_from (value : String) -> Result < Self , Self :: Error > { match DnsName :: try_from_string (value) { Ok (dns) => Ok (Self :: DnsName (dns)) , Err (value) => match IpAddr :: try_from (value . as_str ()) { Ok (ip) => Ok (Self :: IpAddress (ip)) , Err (_) => Err (InvalidDnsNameError) , } , } } }
};
}
