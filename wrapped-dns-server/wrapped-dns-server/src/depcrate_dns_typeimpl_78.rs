// Generated macro for impl_78 (impl)
macro_rules! Depcrate_dns_typeimpl_78 {
() => {
// Module: crate::dns_type
// Provides: {"impl_78"}
// Dependencies: {}
impl Display for DnsType { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , core :: fmt :: Error > { match self { DnsType :: A => write ! (f , "A") , DnsType :: AAAA => write ! (f , "AAAA") , DnsType :: CNAME => write ! (f , "CNAME") , DnsType :: MX => write ! (f , "MX") , DnsType :: NS => write ! (f , "NS") , DnsType :: PTR => write ! (f , "PTR") , DnsType :: SOA => write ! (f , "SOA") , DnsType :: TXT => write ! (f , "TXT") , DnsType :: ANY => write ! (f , "ANY") , DnsType :: Unknown (n) => write ! (f , "Unknown({n})") , } } }
};
}
