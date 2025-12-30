// Generated macro for Response (struct)
macro_rules! DepcrateResponse {
() => {
// Module: crate
// Provides: {"Response"}
// Dependencies: {}
# [doc = " A parsed Response."] # [doc = ""] # [doc = " See `Request` docs for explanation of optional values."] # [derive (Debug , Eq , PartialEq)] pub struct Response < 'headers , 'buf > { # [doc = " The response minor version, such as `1` for `HTTP/1.1`."] pub version : Option < u8 > , # [doc = " The response code, such as `200`."] pub code : Option < u16 > , # [doc = " The response reason-phrase, such as `OK`."] # [doc = ""] # [doc = " Contains an empty string if the reason-phrase was missing or contained invalid characters."] pub reason : Option < & 'buf str > , # [doc = " The response headers."] pub headers : & 'headers mut [Header < 'buf >] }
};
}
