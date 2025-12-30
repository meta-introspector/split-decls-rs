// Generated macro for DnsResponseCode (enum)
macro_rules! Depcrate_dns_response_codeDnsResponseCode {
() => {
// Module: crate::dns_response_code
// Provides: {"DnsResponseCode"}
// Dependencies: {}
# [doc = " > `RCODE` Response code - this 4 bit field is set as part of responses.  The values have the"] # [doc = " > following interpretation:"] # [doc = " > - `0` No error condition"] # [doc = " > - `1` Format error - The name server was unable to interpret the query."] # [doc = " > - `2` Server failure - The name server was unable to process this query due to a problem with"] # [doc = " >   the name server."] # [doc = " > - `3` Name Error - Meaningful only for responses from an authoritative name server, this code"] # [doc = " >   signifies that the domain name referenced in the query does not exist."] # [doc = " > - `4` Not Implemented - The name server does not support the requested kind of query."] # [doc = " > - `5` Refused - The name server refuses to perform the specified operation for policy reasons."] # [doc = " >   For example, a name server may not wish to provide the information to the particular"] # [doc = " >   requester, or a name server may not wish to perform a particular operation (e.g., zone"] # [doc = " >    transfer) for particular data."] # [doc = " > - `6-15` Reserved for future use."] # [doc = ""] # [doc = " <https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1>"] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub enum DnsResponseCode { NoError , FormatError , ServerFailure , NameError , NotImplemented , Refused , Reserved (u8) , }
};
}
