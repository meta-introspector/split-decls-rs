// Generated macro for Http09Request (struct)
macro_rules! Depcrate_commonHttp09Request {
() => {
// Module: crate::common
// Provides: {"Http09Request"}
// Dependencies: {}
# [doc = " Represents an HTTP/0.9 formatted request."] pub struct Http09Request { url : url :: Url , cardinal : u64 , request_line : String , stream_id : Option < u64 > , response_writer : Option < std :: io :: BufWriter < std :: fs :: File > > , }
};
}
