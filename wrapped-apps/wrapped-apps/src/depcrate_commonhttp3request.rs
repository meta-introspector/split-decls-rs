// Generated macro for Http3Request (struct)
macro_rules! Depcrate_commonHttp3Request {
() => {
// Module: crate::common
// Provides: {"Http3Request"}
// Dependencies: {}
# [doc = " Represents an HTTP/3 formatted request."] struct Http3Request { url : url :: Url , cardinal : u64 , stream_id : Option < u64 > , hdrs : Vec < quiche :: h3 :: Header > , priority : Option < Priority > , response_hdrs : Vec < quiche :: h3 :: Header > , response_body : Vec < u8 > , response_body_max : usize , response_writer : Option < std :: io :: BufWriter < std :: fs :: File > > , }
};
}
