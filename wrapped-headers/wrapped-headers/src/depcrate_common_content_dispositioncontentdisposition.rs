// Generated macro for ContentDisposition (struct)
macro_rules! Depcrate_common_content_dispositionContentDisposition {
() => {
// Module: crate::common::content_disposition
// Provides: {"ContentDisposition"}
// Dependencies: {}
# [doc = " A `Content-Disposition` header, (re)defined in [RFC6266](https://tools.ietf.org/html/rfc6266)."] # [doc = ""] # [doc = " The Content-Disposition response header field is used to convey"] # [doc = " additional information about how to process the response payload, and"] # [doc = " also can be used to attach additional metadata, such as the filename"] # [doc = " to use when saving the response payload locally."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " content-disposition = \"Content-Disposition\" \":\""] # [doc = "                       disposition-type *( \";\" disposition-parm )"] # [doc = ""] # [doc = " disposition-type    = \"inline\" | \"attachment\" | disp-ext-type"] # [doc = "                       ; case-insensitive"] # [doc = ""] # [doc = " disp-ext-type       = token"] # [doc = ""] # [doc = " disposition-parm    = filename-parm | disp-ext-parm"] # [doc = ""] # [doc = " filename-parm       = \"filename\" \"=\" value"] # [doc = "                     | \"filename*\" \"=\" ext-value"] # [doc = ""] # [doc = " disp-ext-parm       = token \"=\" value"] # [doc = "                     | ext-token \"=\" ext-value"] # [doc = ""] # [doc = " ext-token           = <the characters in token, followed by \"*\">"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::ContentDisposition;"] # [doc = ""] # [doc = " let cd = ContentDisposition::inline();"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct ContentDisposition (HeaderValue) ;
};
}
