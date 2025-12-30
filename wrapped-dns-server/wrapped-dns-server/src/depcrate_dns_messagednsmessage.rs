// Generated macro for DnsMessage (struct)
macro_rules! Depcrate_dns_messageDnsMessage {
() => {
// Module: crate::dns_message
// Provides: {"DnsMessage"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct DnsMessage { pub header : DnsMessageHeader , pub questions : Vec < DnsQuestion > , pub answers : Vec < DnsRecord > , pub name_servers : Vec < DnsRecord > , pub additional : Vec < DnsRecord > , }
};
}
