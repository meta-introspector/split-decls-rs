// Generated macro for impl_66 (impl)
macro_rules! Depcrate_search_outcomeimpl_66 {
() => {
// Module: crate::search::outcome
// Provides: {"impl_66"}
// Dependencies: {}
# [doc = " Access"] impl MetadataCollection { # [doc = " Return an iterator over the contents of the map in an easy-to-consume form."] pub fn iter (& self) -> impl Iterator < Item = (& str , & Metadata) > { self . name_to_meta . iter () . map (| (k , v) | (k . as_str () , v)) } }
};
}
