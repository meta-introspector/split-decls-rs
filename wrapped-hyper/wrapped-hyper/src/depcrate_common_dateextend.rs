// Generated macro for extend (function)
macro_rules! Depcrate_common_dateextend {
() => {
// Module: crate::common::date
// Provides: {"extend"}
// Dependencies: {}
# [cfg (feature = "http1")] pub (crate) fn extend (dst : & mut Vec < u8 >) { CACHED . with (| cache | { dst . extend_from_slice (cache . borrow () . buffer ()) ; }) }
};
}
