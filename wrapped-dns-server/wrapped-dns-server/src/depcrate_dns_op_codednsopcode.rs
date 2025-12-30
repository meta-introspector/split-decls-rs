// Generated macro for DnsOpCode (enum)
macro_rules! Depcrate_dns_op_codeDnsOpCode {
() => {
// Module: crate::dns_op_code
// Provides: {"DnsOpCode"}
// Dependencies: {}
# [doc = " > `OPCODE`  A four bit field that specifies kind of query in this message."] # [doc = " >         This value is set by the originator of a query and copied into"] # [doc = " >         the response.  The values are:"] # [doc = " > - `0` a standard query (`QUERY`)"] # [doc = " > - `1` an inverse query (`IQUERY`)"] # [doc = " > - `2` a server status request (`STATUS`)"] # [doc = " > - `3-15` reserved for future use"] # [doc = ""] # [doc = " <https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1>"] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub enum DnsOpCode { Query , InverseQuery , Status , Reserved (u8) , }
};
}
