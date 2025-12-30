// Generated macro for impl_390 (impl)
macro_rules! Depcrate_common_etagimpl_390 {
() => {
// Module: crate::common::etag
// Provides: {"impl_390"}
// Dependencies: {}
impl FromStr for ETag { type Err = InvalidETag ; fn from_str (src : & str) -> Result < Self , Self :: Err > { let val = src . parse () . map_err (| _ | InvalidETag { _inner : () }) ? ; EntityTag :: from_owned (val) . map (ETag) . ok_or (InvalidETag { _inner : () }) } }
};
}
