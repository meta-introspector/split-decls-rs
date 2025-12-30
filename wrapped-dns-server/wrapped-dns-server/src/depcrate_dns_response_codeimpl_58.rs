// Generated macro for impl_58 (impl)
macro_rules! Depcrate_dns_response_codeimpl_58 {
() => {
// Module: crate::dns_response_code
// Provides: {"impl_58"}
// Dependencies: {}
impl DnsResponseCode { # [must_use] pub fn new (value : u8) -> Self { match value { 0 => DnsResponseCode :: NoError , 1 => DnsResponseCode :: FormatError , 2 => DnsResponseCode :: ServerFailure , 3 => DnsResponseCode :: NameError , 4 => DnsResponseCode :: NotImplemented , 5 => DnsResponseCode :: Refused , other => DnsResponseCode :: Reserved (other) , } } # [must_use] pub fn num (& self) -> u8 { match self { DnsResponseCode :: NoError => 0 , DnsResponseCode :: FormatError => 1 , DnsResponseCode :: ServerFailure => 2 , DnsResponseCode :: NameError => 3 , DnsResponseCode :: NotImplemented => 4 , DnsResponseCode :: Refused => 5 , DnsResponseCode :: Reserved (other) => * other , } } }
};
}
