// Generated macro for Form (struct)
macro_rules! Depcrate_easy_formForm {
() => {
// Module: crate::easy::form
// Provides: {"Form"}
// Dependencies: {}
# [doc = " Multipart/formdata for an HTTP POST request."] # [doc = ""] # [doc = " This structure is built up and then passed to the `Easy::httppost` method to"] # [doc = " be sent off with a request."] pub struct Form { head : * mut curl_sys :: curl_httppost , tail : * mut curl_sys :: curl_httppost , headers : Vec < List > , buffers : Vec < Vec < u8 > > , strings : Vec < CString > , }
};
}
