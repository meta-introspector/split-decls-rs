// Generated macro for process_datagram (function)
macro_rules! Depcrate_serverprocess_datagram {
() => {
// Module: crate::server
// Provides: {"process_datagram"}
// Dependencies: {}
# [doc = " # Errors"] # [doc = " Returns `Err` when the request is malformed or the server is not configured to answer the"] # [doc = " request."] # [allow (clippy :: implicit_hasher)] pub fn process_datagram (bytes : & mut FixedBuf < 512 > , handler : & impl Fn (& DnsQuestion) -> Vec < DnsRecord > ,) -> Result < FixedBuf < 512 > , DnsError > { let request = DnsMessage :: read (bytes) ? ; let response = process_request (& request , & handler) ? ; let mut out : FixedBuf < 512 > = FixedBuf :: new () ; response . write (& mut out) ? ; Ok (out) }
};
}
