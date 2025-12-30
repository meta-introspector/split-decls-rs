// Generated macro for impl_62 (impl)
macro_rules! Depcrate_server_nameimpl_62 {
() => {
// Module: crate::server_name
// Provides: {"impl_62"}
// Dependencies: {}
# [doc = " Attempt to make a ServerName from a string by parsing as a DNS name or IP address."] impl < 'a > TryFrom < & 'a str > for ServerName < 'a > { type Error = InvalidDnsNameError ; fn try_from (s : & 'a str) -> Result < Self , Self :: Error > { match DnsName :: try_from (s) { Ok (dns) => Ok (Self :: DnsName (dns)) , Err (InvalidDnsNameError) => match IpAddr :: try_from (s) { Ok (ip) => Ok (Self :: IpAddress (ip)) , Err (_) => Err (InvalidDnsNameError) , } , } } }
};
}
