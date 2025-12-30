// Generated macro for Part (struct)
macro_rules! Depcrate_easy_formPart {
() => {
// Module: crate::easy::form
// Provides: {"Part"}
// Dependencies: {}
# [doc = " One part in a multipart upload, added to a `Form`."] pub struct Part < 'form , 'data > { form : & 'form mut Form , name : & 'data str , array : Vec < curl_sys :: curl_forms > , error : Option < FormError > , }
};
}
