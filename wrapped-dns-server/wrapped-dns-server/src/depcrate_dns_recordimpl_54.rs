// Generated macro for impl_54 (impl)
macro_rules! Depcrate_dns_recordimpl_54 {
() => {
// Module: crate::dns_record
// Provides: {"impl_54"}
// Dependencies: {}
impl Debug for DnsRecord { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , core :: fmt :: Error > { match self { DnsRecord :: A (name , addr) => write ! (f , "DnsRecord::A({name},{addr})") , DnsRecord :: AAAA (name , addr) => write ! (f , "DnsRecord::AAAA({name},{addr})") , DnsRecord :: CNAME (name , target) => write ! (f , "DnsRecord::CNAME({name},{target})") , DnsRecord :: NS (name , target) => write ! (f , "DnsRecord::NS({name},{target})") , DnsRecord :: TXT (name , strings) => { write ! (f , "DnsRecord::TXT({name},['") ? ; let mut first = true ; for string in strings { if first { first = false ; write ! (f , "{string}") ? ; } else { write ! (f , "', '{string}") ? ; } } write ! (f , "'])") } DnsRecord :: Unknown (name , typ) => write ! (f , "DnsRecord::Unknown({name},{typ})") , } } }
};
}
