// Generated macro for Request (struct)
macro_rules! DepcrateRequest {
() => {
// Module: crate
// Provides: {"Request"}
// Dependencies: {}
# [doc = " A parsed Request."] # [doc = ""] # [doc = " The optional values will be `None` if a parse was not complete, and did not"] # [doc = " parse the associated property. This allows you to inspect the parts that"] # [doc = " could be parsed, before reading more, in case you wish to exit early."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let buf = b\"GET /404 HTTP/1.1\\r\\nHost:\";"] # [doc = " let mut headers = [httparse::EMPTY_HEADER; 16];"] # [doc = " let mut req = httparse::Request::new(&mut headers);"] # [doc = " let res = req.parse(buf).unwrap();"] # [doc = " if res.is_partial() {"] # [doc = "     match req.path {"] # [doc = "         Some(ref path) => {"] # [doc = "             // check router for path."] # [doc = "             // /404 doesn't exist? we could stop parsing"] # [doc = "         },"] # [doc = "         None => {"] # [doc = "             // must read more and parse again"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [derive (Debug , Eq , PartialEq)] pub struct Request < 'headers , 'buf > { # [doc = " The request method, such as `GET`."] pub method : Option < & 'buf str > , # [doc = " The request path, such as `/about-us`."] pub path : Option < & 'buf str > , # [doc = " The request minor version, such as `1` for `HTTP/1.1`."] pub version : Option < u8 > , # [doc = " The request headers."] pub headers : & 'headers mut [Header < 'buf >] }
};
}
