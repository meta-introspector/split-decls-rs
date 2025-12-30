// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl Http3Req { pub fn new (method : & str , url : & url :: Url , body : Option < Vec < u8 > > , expect_resp_hdrs : Option < Vec < Header > > ,) -> Http3Req { let mut path = String :: from (url . path ()) ; if let Some (query) = url . query () { path . push ('?') ; path . push_str (query) ; } let mut hdrs = vec ! [Header :: new (b":method" , method . as_bytes ()) , Header :: new (b":scheme" , url . scheme () . as_bytes ()) , Header :: new (b":authority" , url . host_str () . unwrap () . as_bytes ()) , Header :: new (b":path" , path . as_bytes ()) , Header :: new (b"user-agent" , USER_AGENT) ,] ; if let Some (body) = & body { hdrs . push (Header :: new (b"content-length" , body . len () . to_string () . as_bytes () ,)) ; } Http3Req { url : url . clone () , hdrs , body , expect_resp_hdrs , resp_hdrs : Vec :: new () , resp_body : Vec :: new () , reset_stream_code : None , } } # [doc = " Add a new [`Header`] to the request. If the request already contains a"] # [doc = " header with the new header's name, the existing header's value will"] # [doc = " be replaced with that of the new one."] pub fn add_or_replace_header (header_list : & mut Vec < Header > , new_header : Header ,) { if let Some (hdr_in_list) = find_header (header_list , & new_header) { * hdr_in_list = new_header ; return ; } header_list . push (new_header . clone ()) ; } }
};
}
