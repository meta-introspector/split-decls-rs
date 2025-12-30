// Generated macro for macro_253 (macro)
macro_rules! Depcrate_cmsmacro_253 {
() => {
// Module: crate::cms
// Provides: {"macro_253"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: CMS_ContentInfo ; fn drop = ffi :: CMS_ContentInfo_free ; # [doc = " High level CMS wrapper"] # [doc = ""] # [doc = " CMS supports nesting various types of data, including signatures, certificates,"] # [doc = " encrypted data, smime messages (encrypted email), and data digest.  The ContentInfo"] # [doc = " content type is the encapsulation of all those content types.  [`RFC 5652`] describes"] # [doc = " CMS and OpenSSL follows this RFC's implementation."] # [doc = ""] # [doc = " [`RFC 5652`]: https://tools.ietf.org/html/rfc5652#page-6"] pub struct CmsContentInfo ; # [doc = " Reference to [`CMSContentInfo`]"] # [doc = ""] # [doc = " [`CMSContentInfo`]:struct.CmsContentInfo.html"] pub struct CmsContentInfoRef ; }
};
}
