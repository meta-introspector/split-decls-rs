// Generated macro for DnsType (enum)
macro_rules! Depcrate_dns_typeDnsType {
() => {
// Module: crate::dns_type
// Provides: {"DnsType"}
// Dependencies: {}
# [doc = " > TYPE fields are used in resource records.  Note that these types are a subset of QTYPEs."] # [doc = ""] # [doc = " <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.2>"] # [doc = ""] # [doc = " > A record type is defined to store a host's IPv6 address.  A host that has more than one"] # [doc = " > IPv6 address must have more than one such record."] # [doc = ""] # [doc = " <https://datatracker.ietf.org/doc/html/rfc3596#section-2>"] # [doc = ""] # [doc = " > QTYPE fields appear in the question part of a query.  QTYPES are a superset of TYPEs, hence"] # [doc = " > all TYPEs are valid QTYPEs."] # [doc = ""] # [doc = " <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.3>"] # [derive (Clone , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] pub enum DnsType { # [doc = " IPv4 address"] A , # [doc = " IPv6 address"] AAAA , # [doc = " The canonical name for an alias"] CNAME , # [doc = " Mail exchange"] MX , # [doc = " Authoritative name server"] NS , # [doc = " Domain name pointer"] PTR , # [doc = " Marks the start of a zone of authority"] SOA , # [doc = " Text string"] TXT , ANY , Unknown (u16) , }
};
}
