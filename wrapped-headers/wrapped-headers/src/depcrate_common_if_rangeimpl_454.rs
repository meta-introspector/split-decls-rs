// Generated macro for impl_454 (impl)
macro_rules! Depcrate_common_if_rangeimpl_454 {
() => {
// Module: crate::common::if_range
// Provides: {"impl_454"}
// Dependencies: {}
impl IfRange { # [doc = " Create an `IfRange` header with an entity tag."] pub fn etag (tag : ETag) -> IfRange { IfRange (IfRange_ :: EntityTag (tag . 0)) } # [doc = " Create an `IfRange` header with a date value."] pub fn date (time : SystemTime) -> IfRange { IfRange (IfRange_ :: Date (time . into ())) } # [doc = " Checks if the resource has been modified, or if the range request"] # [doc = " can be served."] pub fn is_modified (& self , etag : Option < & ETag > , last_modified : Option < & LastModified >) -> bool { match self . 0 { IfRange_ :: Date (since) => last_modified . map (| time | since < time . 0) . unwrap_or (true) , IfRange_ :: EntityTag (ref entity) => { etag . map (| etag | ! etag . 0 . strong_eq (entity)) . unwrap_or (true) } } } }
};
}
