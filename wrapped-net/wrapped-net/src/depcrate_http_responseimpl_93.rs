// Generated macro for impl_93 (impl)
macro_rules! Depcrate_http_responseimpl_93 {
() => {
// Module: crate::http::response
// Provides: {"impl_93"}
// Dependencies: {}
impl ResponseBuilder { # [doc = " Creates a new response object which defaults to status 200"] # [doc = " for other status codes, call Self.status(400)"] pub fn new () -> Self { Self :: default () } # [doc = " Replace _all_ the headers."] pub fn headers (mut self , headers : Headers) -> Self { self . headers = headers ; self } # [doc = " Sets a header."] pub fn header (self , key : & str , value : & str) -> Self { self . headers . set (key , value) ; self } # [doc = " Set the status code"] pub fn status (mut self , status : u16) -> Self { self . options . status (status) ; self } # [doc = " Set the status text"] pub fn status_text (mut self , status_text : & str) -> Self { self . options . status_text (status_text) ; self } # [doc = " A convenience method to set JSON as response body"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This method also sets the `Content-Type` header to `application/json`"] # [cfg (feature = "json")] # [cfg_attr (docsrs , doc (cfg (feature = "json")))] pub fn json < T : serde :: Serialize + ? Sized > (self , value : & T) -> Result < Response , Error > { let json = serde_json :: to_string (value) ? ; self . header ("Content-Type" , "application/json") . body (Some (json . as_str ())) } # [doc = " Set the response body and return the response"] pub fn body < T > (mut self , data : T) -> Result < Response , Error > where T : IntoRawResponse , { self . options . headers (& self . headers . into_raw ()) ; let init = self . options ; data . into_raw (init) . map (Response) . map_err (js_to_error) } }
};
}
