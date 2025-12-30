// Generated macro for DnsClass (enum)
macro_rules! Depcrate_dns_classDnsClass {
() => {
// Module: crate::dns_class
// Provides: {"DnsClass"}
// Dependencies: {}
# [doc = " > `CLASS` fields appear in resource records.  The following `CLASS` mnemonics and values are"] # [doc = " > defined:"] # [doc = " >"] # [doc = " > - `IN` 1 the Internet"] # [doc = " > - `CS` 2 the CSNET class (Obsolete - used only for examples in some obsolete RFCs)"] # [doc = " > - `CH` 3 the CHAOS class"] # [doc = " > - `HS` 4 Hesiod [Dyer 87]"] # [doc = " >"] # [doc = " >"] # [doc = " > `QCLASS` fields appear in the question section of a query.  `QCLASS` values are a superset of"] # [doc = " > `CLASS` values; every `CLASS` is a valid `QCLASS`.  In addition to `CLASS` values, the following"] # [doc = " > `QCLASSes` are defined:"] # [doc = " >"] # [doc = " > - `*` 255 any class"] # [doc = ""] # [doc = " <https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.4>"] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub enum DnsClass { Internet , Any , Unknown (u16) , }
};
}
