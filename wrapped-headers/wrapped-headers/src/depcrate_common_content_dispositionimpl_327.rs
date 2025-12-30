// Generated macro for impl_327 (impl)
macro_rules! Depcrate_common_content_dispositionimpl_327 {
() => {
// Module: crate::common::content_disposition
// Provides: {"impl_327"}
// Dependencies: {}
impl ContentDisposition { # [doc = " Construct a `Content-Disposition: inline` header."] pub fn inline () -> ContentDisposition { ContentDisposition (HeaderValue :: from_static ("inline")) } # [doc = " Check if the disposition-type is `inline`."] pub fn is_inline (& self) -> bool { self . get_type () == "inline" } # [doc = " Check if the disposition-type is `attachment`."] pub fn is_attachment (& self) -> bool { self . get_type () == "attachment" } # [doc = " Check if the disposition-type is `form-data`."] pub fn is_form_data (& self) -> bool { self . get_type () == "form-data" } fn get_type (& self) -> & str { self . 0 . to_str () . unwrap_or ("") . split (';') . next () . expect ("split always has at least 1 item") } }
};
}
