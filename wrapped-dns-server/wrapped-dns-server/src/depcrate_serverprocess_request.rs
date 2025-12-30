// Generated macro for process_request (function)
macro_rules! Depcrate_serverprocess_request {
() => {
// Module: crate::server
// Provides: {"process_request"}
// Dependencies: {}
# [doc = " # Errors"] # [doc = " Returns `Err` when the request is malformed or the server is not configured to answer the"] # [doc = " request."] pub fn process_request (request : & DnsMessage , handler : & impl Fn (& DnsQuestion) -> Vec < DnsRecord > ,) -> Result < DnsMessage , DnsError > { if request . header . is_response { return Err (DnsError :: NotARequest) ; } if request . header . op_code != DnsOpCode :: Query { return Err (DnsError :: InvalidOpCode) ; } let question = request . questions . first () . ok_or (DnsError :: NoQuestion) ? ; let records = handler (question) ; request . answer_response (records) }
};
}
