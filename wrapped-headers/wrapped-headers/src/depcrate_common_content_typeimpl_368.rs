// Generated macro for impl_368 (impl)
macro_rules! Depcrate_common_content_typeimpl_368 {
() => {
// Module: crate::common::content_type
// Provides: {"impl_368"}
// Dependencies: {}
impl std :: str :: FromStr for ContentType { type Err = Error ; fn from_str (s : & str) -> Result < ContentType , Self :: Err > { s . parse :: < Mime > () . map (| m | m . into ()) . map_err (| _ | Error :: invalid ()) } }
};
}
